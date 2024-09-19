use candid::CandidType;
use serde::{Deserialize, Serialize};

#[derive(CandidType, Deserialize, Debug, Clone, Default, Serialize)]
pub struct VoteMessage {
    pub proposal_id: u32,
    pub space_id: u32,
    pub option_id: u32,
    pub address: String,
}

#[derive(CandidType, Deserialize, Debug, Clone, Default)]
pub struct VoteData {
    pub signature: String,
    pub message: VoteMessage,
}
