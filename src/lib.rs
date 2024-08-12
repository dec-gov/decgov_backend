use candid::{CandidType, Deserialize};
use serde_bytes::ByteBuf;
use std::cell::RefCell;
use std::rc::Rc;
use std::str::FromStr;
use ic_cdk_macros::query;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // static SPACES: RefCell<StableBTreeMap<String, Space, Memory>> = RefCell::new(
    //     StableBTreeMap::init(
    //         MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
    //     )
    // );

    static SPACES: RefCell<Vec<Space>> = RefCell::new(Vec::new());
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Space {
    pub id: u32,
    pub name: String,
    pub icon_link: String,
    pub website_link: String,
    pub vote_delay: u32,
    pub vote_duration: u32,
    pub min_vote_role: u32,
    pub min_vote_power: u64,
    pub quorum: u32,
}

#[query]
fn get_spaces() -> Vec<Space> {
    SPACES.with(|proposals_ref| proposals_ref.borrow().clone())
}

#[query]
fn get_space(id: u32) -> Option<Space> {
    SPACES.with(|proposals_ref| {
        proposals_ref
            .borrow()
            .iter()
            .find(|space| space.id == id)
            .cloned()
    })
}

#[query]
fn insert_space(
    name: String,
    icon_link: String,
    website_link: String,
    vote_delay: u32,
    vote_duration: u32,
    min_vote_role: u32,
    min_vote_power: u64,
    quorum: u32,
)-> Space {
    SPACES.with(|proposals_ref| {
        let mut proposals = proposals_ref.borrow_mut();

        let id = proposals.len() as u32;
        proposals.push(Space {
            id,
            name,
            icon_link,
            website_link,
            vote_delay,
            vote_duration,
            min_vote_role,
            min_vote_power,
            quorum,
        });

        proposals[id as usize].clone()
    })
}

#[query]
fn update_space(
    id: u32,
    name: String,
    icon_link: String,
    website_link: String,
    vote_delay: u32,
    vote_duration: u32,
    min_vote_role: u32,
    min_vote_power: u64,
    quorum: u32,
) -> Option<Space> {
    SPACES.with(|proposals_ref| {
        let mut proposals = proposals_ref.borrow_mut();

        if let Some(space) = proposals.iter_mut().find(|space| space.id == id) {
            space.name = name;
            space.icon_link = icon_link;
            space.website_link = website_link;
            space.vote_delay = vote_delay;
            space.vote_duration = vote_duration;
            space.min_vote_role = min_vote_role;
            space.min_vote_power = min_vote_power;
            space.quorum = quorum;

            Some(space.clone())
        } else {
            None
        }
    })
}

#[query]
fn delete_space(id: u32) -> Option<Space> {
    SPACES.with(|proposals_ref| {
        let mut proposals = proposals_ref.borrow_mut();

        if let Some(index) = proposals.iter().position(|space| space.id == id) {
            Some(proposals.remove(index))
        } else {
            None
        }
    })
}
