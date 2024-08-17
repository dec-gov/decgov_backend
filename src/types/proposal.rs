use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

use super::proposal_options::ProposalOption;

const MAX_VALUE_SIZE: u32 = 500;

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct Proposal {
   pub id: u32,
   pub title: String,
   pub description: String,
   pub date_created: u64,
   pub mechanism: u32,
   pub space_id: u32,
   pub options: Vec<ProposalOption>,
}


impl Storable for Proposal {
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
