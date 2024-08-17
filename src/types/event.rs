use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct Event {
    pub event_type: u32,
    pub webhook_url: String,
    pub payload: String,
    pub space_id: u32,
}