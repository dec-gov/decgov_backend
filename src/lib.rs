mod types;

use ic_cdk::update;
use ic_cdk_macros::query;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use types::space::Space;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static SPACES: RefCell<StableBTreeMap<u32, Space, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );

    // static SPACES: RefCell<Vec<Space>> = RefCell::new(Vec::new());
}

#[query]
fn get_spaces() -> Option<Vec<Space>> {
    Some(SPACES.with(|p| p.borrow().iter().map(|(_, v)| v.clone()).collect()))
}

#[query]
fn get_space(id: u32) -> Option<Space> {
    SPACES.with(|p| p.borrow().get(&id))
}

#[update]
fn insert_space(
    name: String,
    icon_link: String,
    website_link: String,
    vote_delay: u32,
    vote_duration: u32,
    min_vote_role: u32,
    min_vote_power: u64,
    quorum: u32,
) -> Space {
    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        let id = spaces.len() as u32 + 1;
        let space = Space {
            id,
            name,
            icon_link,
            website_link,
            vote_delay,
            vote_duration,
            min_vote_role,
            min_vote_power,
            quorum,
        };
        spaces.insert(id, space.clone());
        space
    })
}

#[update]
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
    let space = get_space(id);
    if space.is_none() {
        return None;
    }
    // let space = space.unwrap();
    let new_space = Space {
        id,
        name,
        icon_link,
        website_link,
        vote_delay,
        vote_duration,
        min_vote_role,
        min_vote_power,
        quorum,
    };

    delete_space(id);

    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.insert(id, new_space.clone());
    });

    Some(new_space)
}

#[update]
fn delete_space(id: u32) -> Option<Space> {
    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.remove(&id)
    })
}

ic_cdk::export_candid!();
