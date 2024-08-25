use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Event {
    pub event_type: EventType,
    pub webhook_url: String,
    pub payload: String,
    pub space_id: u32,
}

#[derive(CandidType, Deserialize, PartialEq, Eq)]
pub enum EventType {
    ProposalCreated,
    ProposalEnded,
    Vote,
}