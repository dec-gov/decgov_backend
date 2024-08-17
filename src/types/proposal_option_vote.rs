use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

const MAX_VALUE_SIZE: u32 = 1000;

#[derive(CandidType, Deserialize, Debug, Clone)]

pub struct ProposalOptionVote {
    pub id: u32,
    pub user_address: String,
    pub vote_type: u32,
    pub timestamp: u32,
    pub signature: String,
    pub voting_power: u128,
    pub option_id: u32,
}

impl Storable for ProposalOptionVote {
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
