use candid::CandidType;
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct WebhookEvent {
    pub webhook_url: String,
    pub payload: String,
}