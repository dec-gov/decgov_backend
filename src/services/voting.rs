use core::panic;
use std::collections::HashMap;

use candid::Nat;
use ethers_core::types::{Address, Signature};
use ic_cdk::{
    api::management_canister::http_request::{
        http_request, CanisterHttpRequestArgument, HttpHeader, HttpMethod,
    },
    update,
};

use crate::{
    get_events_by_space, get_proposal, get_space, get_strategies, insert_vote,
    types::{
        event::{Event, EventData, EventTrigger},
        strategy::{Strategy, StrategyData},
        vote::VoteData,
        webhook_event::WebhookEvent,
    },
};

use super::eth_rpc::eth_call;

#[update]
async fn vote(data: VoteData) -> Result<Nat, String> {
    let message_json = serde_json::to_string(&data.message).unwrap();
    let signature = data.signature.parse::<Signature>().unwrap();

    let recovered_address = signature.recover(message_json).unwrap();
    let parsed_address = data.message.address.parse::<Address>().unwrap();

    if recovered_address != parsed_address {
        return Err("Invalid signature".to_owned());
    }

    if !data.message.address.starts_with("0x") {
        return Err("Only Ethereum address is supported for now".into());
    }

    let voting_power = get_voting_power(&recovered_address, data.message.space_id, None)
        .await
        .unwrap();

    if voting_power == 0 as u32 {
        return Err("Insufficient voting power".to_owned());
    }

    let space = get_space(data.message.space_id);
    let proposal = get_proposal(data.message.space_id, data.message.proposal_id);

    if space.is_none() || proposal.is_none() {
        return Err("Invalid vote message".to_owned());
    }

    let (space, proposal) = (space.unwrap(), proposal.unwrap());
    let vote_timestamp = ic_cdk::api::time() / 1_000_000_000;

    // date_created = 10s
    // vote_delay = 3s
    // vote_duration = 60s
    // 0s -------------- 10s -------------- 13s ------------- 73s ------------- inf s
    //           proposal created       voting available    voting finished
    if (proposal.date_created + space.vote_delay as u64) > vote_timestamp
        || (proposal.date_created + space.vote_duration as u64 + space.vote_delay as u64)
            < vote_timestamp
    {
        return Err("Voting is not available for this proposal".to_owned());
    }

    insert_vote(
        data.message.space_id,
        data.message.proposal_id,
        data.message.option_id,
        data.message.address.clone(),
        0,
        vote_timestamp,
        data.signature,
        voting_power.clone(),
    );

    trigger_events(
        data.message.space_id,
        EventTrigger::Vote,
        HashMap::from([
            ("power", voting_power.to_string()),
            ("address", data.message.address.clone()),
        ]),
    )
    .await;

    Ok(voting_power)
}

#[update]
async fn voting_power(
    address: String,
    space_id: u32,
    block_height: Option<String>,
) -> Result<Nat, String> {
    let voting_power = get_voting_power(&address.parse().unwrap(), space_id, block_height)
        .await
        .unwrap();

    Ok(voting_power)
}

async fn get_voting_power(
    address: &Address,
    space_id: u32,
    block_height: Option<String>,
) -> Result<Nat, String> {
    let strategies: Vec<Strategy> = get_strategies(space_id)
        .unwrap()
        .into_iter()
        .filter(|s| s.space_id == space_id)
        .collect();

    let mut total_voting_power = Nat::from(0 as u32);

    for strategy in strategies {
        let voting_power = call_strategy(&address, &strategy, block_height.clone()).await?;
        total_voting_power += voting_power;
    }

    return Ok(total_voting_power);
}

async fn trigger_events(
    space_id: u32,
    event_trigger: EventTrigger,
    event_data: HashMap<&str, String>,
) {
    let events: Vec<Event> = get_events_by_space(space_id).expect("Invalid space id");

    for event in events.into_iter() {
        if event.event_trigger != event_trigger {
            continue;
        }

        match event.data {
            EventData::Webhook(data) => handle_webhook_event(data, &event_data).await,
            EventData::Evm(_) => panic!("Evm events are not implemented yet"),
        }
    }
}

async fn handle_webhook_event(event: WebhookEvent, event_data: &HashMap<&str, String>) {
    let mut payload = event.payload;

    for (key, value) in event_data.iter() {
        let new_key = format!("${{{}}}", key);
        payload = payload.replace(&new_key, &value);
    }

    let json_utf8: Vec<u8> = payload.into_bytes();
    let request_body: Option<Vec<u8>> = Some(json_utf8);

    let request = CanisterHttpRequestArgument {
        url: event.webhook_url,
        method: HttpMethod::POST,
        max_response_bytes: None,
        headers: vec![HttpHeader {
            name: String::from("Content-Type"),
            value: String::from("application/json"),
        }],
        body: request_body,
        transform: None,
    };

    http_request(request, 2_000_000_000).await.unwrap();
}

async fn call_strategy(
    address: &Address,
    strategy: &Strategy,
    block_height: Option<String>,
) -> Result<Nat, String> {
    if let StrategyData::Evm(ref evm_strategy) = strategy.data {
        let str_address = format!("{:x}", &address).replace("0x", "");
        let data = evm_strategy
            .bytecode
            .clone()
            .replace("$voterAddress", &str_address);

        let response = eth_call(evm_strategy.contract_address.clone(), data, block_height).await;

        if let Ok(value) = response {
            if value == "0x" {
                return Ok(Nat::from(0 as u8));
            }

            Ok(Nat::from(
                u128::from_str_radix(&value.trim_start_matches("0x"), 16).unwrap(),
            ))
        } else {
            Err("Unable to parse response from contract call".into())
        }
    } else {
        Err("Only EVM strategies are supported for now".into())
    }
}
