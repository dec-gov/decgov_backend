use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::proposal_option_vote::ProposalOptionVote;

const MAX_VALUE_SIZE: u32 = 500;

#[derive(CandidType, Deserialize, Debug, Clone)]

pub struct ProposalOption {
    pub id: u32,
    pub name: String,
    pub proposal_id: u32,
    pub on_win_contract_address: String,
    pub on_win_bytecode: String,
    pub on_win_chain_id: u32,
    pub votes: Vec<ProposalOptionVote>,
}

impl Storable for ProposalOption {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
}
