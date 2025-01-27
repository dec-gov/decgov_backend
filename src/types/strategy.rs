use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::{btc_strategy::BtcStrategy, evm_strategy::EvmStrategy};

const MAX_VALUE_SIZE: u32 = 1000;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Strategy {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub space_id: u32,
    pub data: StrategyData
}

impl Storable for Strategy {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum StrategyData {
    Evm(EvmStrategy),
    Btc(BtcStrategy)
}