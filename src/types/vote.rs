use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Debug, Clone, Default, Serialize)]
pub struct VoteMessage {
    pub address: String,
    pub proposal_id: u32,
    pub space_id: u32,
    pub option_id: u32,
    pub block_height: Option<String>,
}

#[derive(CandidType, Deserialize, Debug, Clone, Default)]
pub struct VoteData {
    pub signature: String,
    pub message: VoteMessage,
}
