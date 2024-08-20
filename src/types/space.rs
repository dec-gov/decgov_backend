use candid::{CandidType, Decode, Deserialize, Encode, Nat};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::proposal::Proposal;

const MAX_VALUE_SIZE: u32 = 500;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Space {
    pub id: u32,
    pub name: String,
    pub icon_link: String,
    pub website_link: String,
    pub vote_delay: u32,
    pub vote_duration: u32,
    pub min_vote_role: u32,
    pub min_vote_power: Nat,
    pub quorum: Nat,
    pub proposals: Vec<Proposal>,

}

impl Storable for Space {
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
