use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::{evm_event::EvmEvent, webhook_event::WebhookEvent};

const MAX_VALUE_SIZE: u32 = 1000;

#[derive(CandidType, Deserialize, Debug, Clone)]

pub struct Event {
    pub event_type: EventType,
    pub space_id: u32,
    pub data: EventData
}

impl Storable for Event {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum EventType {
    ProposalCreated,
    ProposalEnded,
    Vote,
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub enum EventData {
    Evm(EvmEvent),
    Webhook(WebhookEvent)
}
