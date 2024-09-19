use ethers_core::{
    k256::{
        ecdsa::{RecoveryId, Signature, VerifyingKey},
        elliptic_curve::sec1::ToEncodedPoint,
        PublicKey,
    },
    types::Bytes,
    utils::keccak256,
};
use ic_stable_structures::Storable;
use std::{cell::RefCell, str::FromStr};

use candid::{Nat, Principal};
use ethers_core::{
    abi::{Abi, Address, FunctionExt, Token},
    types::{Eip1559TransactionRequest, NameOrAddress, H160, U256},
};
use hex::FromHexError;
use ic_cdk::api::{
    call::{call_with_payment, call_with_payment128, CallResult},
    management_canister::ecdsa::{
        ecdsa_public_key, sign_with_ecdsa, EcdsaKeyId, EcdsaPublicKeyArgument,
        SignWithEcdsaArgument,
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    types::eth_rpc::{
        BlockTag, EthMainnetService, EthSepoliaService, GetTransactionCountArgs,
        GetTransactionCountResult, MultiGetTransactionCountResult, MultiSendRawTransactionResult,
        RequestResult, RpcConfig, RpcService, RpcServices, SendRawTransactionResult,
        SendRawTransactionStatus,
    },
    ECDSA_KEY,
};

pub const CANISTER_ID: Principal =
    Principal::from_slice(b"\x00\x00\x00\x00\x02\x30\x00\xCC\x01\x01"); // 7hfb6-caaaa-aaaar-qadga-cai

thread_local! {
    static SELF_ETH_ADDRESS: RefCell<Option<String>> =
        RefCell::new(None);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    id: u64,
    jsonrpc: String,
    method: String,
    params: (EthCallParams, String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct EthCallParams {
    to: String,
    data: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcResult {
    result: Option<String>,
    error: Option<JsonRpcError>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonRpcError {
    code: isize,
    message: String,
}

fn ecdsa_key_id() -> EcdsaKeyId {
    EcdsaKeyId {
        curve: ic_cdk::api::management_canister::ecdsa::EcdsaCurve::Secp256k1,
        // name: ECDSA_KEY.with(|key| key.borrow().clone()),
        name: "dfx_test_key".to_owned(),
    }
}

async fn pubkey_and_signature(message_hash: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    // Fetch the pubkey and the signature concurrently to reduce latency.
    let (pubkey, response) = futures::join!(
        ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: vec![],
            key_id: ecdsa_key_id()
        }),
        sign_with_ecdsa(SignWithEcdsaArgument {
            message_hash,
            derivation_path: vec![],
            key_id: ecdsa_key_id(),
        })
    );
    (
        pubkey.unwrap().0.public_key,
        response.expect("failed to sign the message").0.signature,
    )
}

pub async fn sign_transaction(to: &Address, data: &Bytes, chain_id: u64) -> String {
    use ethers_core::types::Signature;

    let tx = Eip1559TransactionRequest {
        from: None,
        to: Some(NameOrAddress::Address(*to)),
        gas: Some(300_000.into()),
        max_priority_fee_per_gas: None,
        data: Some(data.clone()),
        chain_id: Some(chain_id.into()),
        ..Default::default()
    };

    let mut unsigned_tx_bytes = tx.rlp().to_vec();
    unsigned_tx_bytes.insert(0, 2);

    let tx_hash = keccak256(&unsigned_tx_bytes);

    let (pubkey, signature) = pubkey_and_signature(tx_hash.to_vec()).await;

    let signature = Signature {
        v: y_parity(&tx_hash.to_bytes(), &signature, &pubkey),
        r: U256::from_big_endian(&signature[0..32]),
        s: U256::from_big_endian(&signature[32..64]),
    };

    let mut signed_tx_bytes = tx.rlp_signed(&signature).to_vec();
    signed_tx_bytes.insert(0, 2);

    format!("0x{}", hex::encode(signed_tx_bytes))
}

pub async fn call(data: String) -> Result<String, String> {
    let (res,): (MultiSendRawTransactionResult,) = call_with_payment(
        CANISTER_ID,
        "eth_sendRawTransaction",
        (
            RpcServices::EthSepolia(None),
            None::<RpcConfig>,
            data.clone(),
        ),
        2_000_000_000_000,
    )
    .await
    .unwrap();

    match res {
        MultiSendRawTransactionResult::Consistent(SendRawTransactionResult::Ok(
            SendRawTransactionStatus::Ok(txid),
        )) => match txid {
            Some(id) => Ok(id),
            None => {
                Err("Transaction ID was missing despite successful transaction status.".to_string())
            }
        },

        other => Err(format!("call: {:?}, error: {:?}", &data, other)),
    }
}

/// Computes the parity bit allowing to recover the public key from the signature.
fn y_parity(prehash: &[u8], sig: &[u8], pubkey: &[u8]) -> u64 {
    let orig_key = VerifyingKey::from_sec1_bytes(pubkey).expect("failed to parse the pubkey");
    let signature = Signature::try_from(sig).unwrap();
    for parity in [0u8, 1] {
        let recid = RecoveryId::try_from(parity).unwrap();
        let recovered_key = VerifyingKey::recover_from_prehash(prehash, &signature, recid)
            .expect("failed to recover key");
        if recovered_key == orig_key {
            return parity as u64;
        }
    }

    panic!(
        "failed to recover the parity bit from a signature; sig: {}, pubkey: {}",
        hex::encode(sig),
        hex::encode(pubkey)
    )
}

async fn next_id() -> Nat {
    let res: CallResult<(MultiGetTransactionCountResult,)> = call_with_payment(
        CANISTER_ID,
        "eth_getTransactionCount",
        (
            RpcService::EthSepolia(EthSepoliaService::BlockPi),
            None::<RpcConfig>,
            GetTransactionCountArgs {
                address: get_self_eth_address().await,
                block: BlockTag::Latest,
            },
        ),
        2_000_000_000,
    )
    .await;
    match res {
        Ok((MultiGetTransactionCountResult::Consistent(GetTransactionCountResult::Ok(id)),)) => {
            id.into()
        }
        Ok((inconsistent,)) => ic_cdk::trap(&format!("Inconsistent: {inconsistent:?}")),
        Err(err) => ic_cdk::trap(&format!("{:?}", err)),
    }
}

pub async fn eth_call(
    contract_address: String,
    data: String,
    block_height: Option<String>,
) -> Result<String, String> {
    let json_rpc_payload = serde_json::to_string(&JsonRpcRequest {
        id: next_id().await.0.try_into().unwrap(),
        jsonrpc: "2.0".to_string(),
        method: "eth_call".to_string(),
        params: (
            EthCallParams {
                data,
                to: contract_address,
            },
            block_height.unwrap_or("latest".to_string()),
        ),
    })
    .expect("Error while encoding JSON-RPC request");

    let res: CallResult<(RequestResult,)> = call_with_payment128(
        CANISTER_ID,
        "request",
        (
            RpcService::EthSepolia(EthSepoliaService::BlockPi),
            json_rpc_payload,
            2048_u64,
        ),
        2_000_000_000,
    )
    .await;

    match res {
        Ok((RequestResult::Ok(ok),)) => {
            let json: JsonRpcResult =
                serde_json::from_str(&ok).expect("JSON was not well-formatted");

            if let Some(result) = json.result {
                Ok(result)
            } else {
                Err("".to_owned())
            }
        }

        err => panic!("Response error: {err:?}"),
    }
}

pub async fn get_self_eth_address() -> String {
    if SELF_ETH_ADDRESS.with(|maybe_address| maybe_address.borrow().is_none()) {
        let (pubkey,) = ecdsa_public_key(EcdsaPublicKeyArgument {
            canister_id: None,
            derivation_path: vec![],
            key_id: ecdsa_key_id(),
        })
        .await
        .unwrap();

        let key = PublicKey::from_sec1_bytes(&pubkey.public_key)
            .expect("failed to parse the public key as SEC1");
        let point = key.to_encoded_point(false);
        // we re-encode the key to the decompressed representation.
        let point_bytes = point.as_bytes();
        assert_eq!(point_bytes[0], 0x04);

        let hash = keccak256(&point_bytes[1..]);

        let self_address =
            ethers_core::utils::to_checksum(&Address::from_slice(&hash[12..32]), None);
        SELF_ETH_ADDRESS.with(|maybe_address| *maybe_address.borrow_mut() = Some(self_address));
    }

    SELF_ETH_ADDRESS.with(|maybe_address| maybe_address.borrow().clone().unwrap())
}