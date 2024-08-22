mod services;
mod types;
mod utils;

use candid::{Nat, Principal};
use ic_cdk::update;
use ic_cdk_macros::query;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;
use types::evm_strategy::{self, EvmStrategy};
use types::proposal::Proposal;
use types::proposal_option_vote::ProposalOptionVote;
use types::proposal_options::{InsertProposalOption, ProposalOption};
use types::space::Space;
use types::strategy::Strategy;
use types::vote::VoteData;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static SPACES: RefCell<StableBTreeMap<u32, Space, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
    pub static ECDSA_KEY: RefCell<String> = RefCell::new(String::default());
}

//SPACES

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
    min_vote_power: Nat,
    quorum: Nat,
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
            proposals: Vec::new(),
            strategies: Vec::new(),
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
    min_vote_power: Nat,
    quorum: Nat,
) -> Option<Space> {
    let space = get_space(id);
    if space.is_none() {
        return None;
    }
    let unwrapped_space = space.unwrap();
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
        proposals: unwrapped_space.proposals,
        strategies: unwrapped_space.strategies,
    };

    delete_space(id);

    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.insert(id, new_space.clone());
    });

    Some(new_space)
}

#[update]
fn update_space_proposals(id: u32, proposals: Vec<Proposal>) {
    let space = get_space(id);
    if space.is_none() {
        return;
    }
    let space = space.unwrap();
    let mut new_space = space.clone();
    new_space.proposals = proposals;

    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.insert(id, new_space.clone());
    });
}

#[update]
fn delete_space(id: u32) -> Option<Space> {
    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.remove(&id)
    })
}

//PROPOSALS
#[update]
fn insert_proposal(
    space_id: u32,
    title: String,
    description: String,
    mechanism: u32,
    options: Vec<InsertProposalOption>,
) -> Option<Proposal> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let id = proposals.len() as u32 + 1;
    // Convert nanoseconds to seconds
    let date_created = ic_cdk::api::time() / 1_000_000_000;

    let mut new_options: Vec<ProposalOption> = Vec::new();
    let mut option_id = 1;
    
    for option in options.iter() {
     new_options.push(ProposalOption {
        id: option_id,
        name: option.name.clone(),
        on_win_contract_address:"".to_string(),
        on_win_bytecode: "".to_string(),
        on_win_chain_id: 0,
        proposal_id: id,
        votes: Vec::new(),
     });
     option_id += 1;
    }


    let new_proposal = types::proposal::Proposal {
        id,
        title,
        description,
        date_created,
        mechanism,
        space_id,
        options: new_options,
    };

    proposals.push(new_proposal.clone());
    update_space_proposals(space_id, proposals);

    Some(new_proposal)
}

#[query]
fn get_proposals(space_id: u32) -> Option<Vec<Proposal>> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    Some(space.unwrap().proposals)
}

#[query]
fn get_proposal(space_id: u32, proposal_id: u32) -> Option<Proposal> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    Some(proposal.unwrap().clone())
}

#[update]
fn update_proposal(
    space_id: u32,
    proposal_id: u32,
    title: String,
    description: String,
    mechanism: u32,
) -> Option<Proposal> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let new_proposal = Proposal {
        id: proposal_id,
        title,
        description,
        date_created: proposal.date_created,
        mechanism,
        space_id,
        options: proposal.options.clone(),
    };

    let index = proposals.iter().position(|p| p.id == proposal_id).unwrap();
    proposals[index] = new_proposal.clone();
    update_space_proposals(space_id, proposals);

    Some(new_proposal)
}

#[update]
fn delete_proposal(space_id: u32, proposal_id: u32) -> Option<Proposal> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let index = proposals.iter().position(|p| p.id == proposal_id).unwrap();
    let mut new_proposals = proposals.clone();
    new_proposals.remove(index);
    update_space_proposals(space_id, new_proposals);

    Some(proposal.clone())
}

//Options must not be editable from outside
fn update_proposal_options(
    space_id: u32,
    proposal_id: u32,
    options: Vec<ProposalOption>,
) -> Option<Proposal> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let new_proposal = Proposal {
        id: proposal_id,
        title: proposal.title.clone(),
        description: proposal.description.clone(),
        date_created: proposal.date_created,
        mechanism: proposal.mechanism,
        space_id,
        options,
    };

    let index = proposals.iter().position(|p| p.id == proposal_id).unwrap();
    proposals[index] = new_proposal.clone();
    update_space_proposals(space_id, proposals);

    Some(new_proposal)
}

//PROPOSAL OPTIONS

#[update]
fn insert_proposal_option(
    space_id: u32,
    proposal_id: u32,
    name: String,
    on_win_contract_address: String,
    on_win_bytecode: String,
    on_win_chain_id: u32,
) -> Option<ProposalOption> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let options = proposal.options.clone();
    let id = options.len() as u32 + 1;
    let new_option = ProposalOption {
        id,
        name,
        on_win_contract_address,
        on_win_bytecode,
        on_win_chain_id,
        proposal_id,
        votes: Vec::new(),
    };

    let mut options = proposal.options.clone();
    options.push(new_option.clone());
    update_proposal_options(space_id, proposal_id, options);

    Some(new_option)
}

#[query]
fn get_proposal_options(space_id: u32, proposal_id: u32) -> Option<Vec<ProposalOption>> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    Some(proposal.unwrap().options.clone())
}

#[query]
fn get_proposal_option(space_id: u32, proposal_id: u32, option_id: u32) -> Option<ProposalOption> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let options = proposal.options.clone();
    let option = options.iter().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    Some(option.unwrap().clone())
}

//Options must not be editable from outside

#[update]
fn delete_proposal_option(
    space_id: u32,
    proposal_id: u32,
    option_id: u32,
) -> Option<ProposalOption> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let options = proposal.options.clone();
    let option = options.iter().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    let option = option.unwrap();
    let index = options.iter().position(|o| o.id == option_id).unwrap();

    let mut new_options = options.clone();
    new_options.remove(index);

    update_proposal_options(space_id, proposal_id, new_options);

    Some(option.clone())
}

#[update]
fn insert_vote(
    space_id: u32,
    proposal_id: u32,
    option_id: u32,
    user_address: String,
    vote_type: u32,
    timestamp: u64,
    signature: String,
    voting_power: Nat,
) -> Option<ProposalOption> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let mut options = proposal.options.clone();
    let option = options.iter_mut().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    let option = option.unwrap();
    let mut votes = option.votes.clone();
    let id = votes.len() as u32 + 1;
    let new_vote = ProposalOptionVote {
        id,
        user_address,
        vote_type,
        timestamp,
        signature,
        voting_power,
        option_id,
    };

    votes.push(new_vote.clone());
    let mut cloned_options = options.clone();
    let index = cloned_options
        .iter()
        .position(|o| o.id == option_id)
        .unwrap();
    cloned_options[index].votes = votes;
    let new_option = cloned_options[index].clone();
    update_proposal_options(space_id, proposal_id, cloned_options);

    Some(new_option)
}

// PROPOSAL OPTION VOTES

#[query]
fn get_votes(space_id: u32, proposal_id: u32, option_id: u32) -> Option<Vec<ProposalOptionVote>> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let options = proposal.options.clone();
    let option = options.iter().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    Some(option.unwrap().votes.clone())
}

#[query]
fn get_vote(
    space_id: u32,
    proposal_id: u32,
    option_id: u32,
    vote_id: u32,
) -> Option<ProposalOptionVote> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let proposals = space.unwrap().proposals;
    let proposal = proposals.iter().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let options = proposal.options.clone();
    let option = options.iter().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    let option = option.unwrap();
    let votes = option.votes.clone();
    let vote = votes.iter().find(|v| v.id == vote_id);
    if vote.is_none() {
        return None;
    }
    Some(vote.unwrap().clone())
}

#[update]
fn update_vote(
    space_id: u32,
    proposal_id: u32,
    option_id: u32,
    vote_id: u32,
    user_address: String,
    vote_type: u32,
    timestamp: u64,
    signature: String,
    voting_power: Nat,
) -> Option<ProposalOptionVote> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let mut options = proposal.options.clone();
    let option = options.iter_mut().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    let option = option.unwrap();
    let mut votes = option.votes.clone();
    let vote = votes.iter_mut().find(|v| v.id == vote_id);
    if vote.is_none() {
        return None;
    }
    let new_vote = ProposalOptionVote {
        id: vote_id,
        user_address,
        vote_type,
        timestamp,
        signature,
        voting_power,
        option_id,
    };

    let index = votes.iter().position(|v| v.id == vote_id).unwrap();
    votes[index] = new_vote.clone();
    let mut cloned_options = options.clone();
    let index = cloned_options
        .iter()
        .position(|o| o.id == option_id)
        .unwrap();
    cloned_options[index].votes = votes;
    update_proposal_options(space_id, proposal_id, cloned_options);

    Some(new_vote)
}

#[update]
fn delete_vote(
    space_id: u32,
    proposal_id: u32,
    option_id: u32,
    vote_id: u32,
) -> Option<ProposalOptionVote> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut proposals = space.unwrap().proposals;
    let proposal = proposals.iter_mut().find(|p| p.id == proposal_id);
    if proposal.is_none() {
        return None;
    }
    let proposal = proposal.unwrap();
    let mut options = proposal.options.clone();
    let option = options.iter_mut().find(|o| o.id == option_id);
    if option.is_none() {
        return None;
    }
    let option = option.unwrap();
    let votes = option.votes.clone();
    let vote = votes.iter().find(|v| v.id == vote_id);
    if vote.is_none() {
        return None;
    }
    let vote = vote.unwrap();
    let index = votes.iter().position(|v| v.id == vote_id).unwrap();
    let mut new_votes = votes.clone();
    new_votes.remove(index);

    let mut cloned_options = options.clone();
    let index = cloned_options
        .iter()
        .position(|o| o.id == option_id)
        .unwrap();
    cloned_options[index].votes = new_votes;
    update_proposal_options(space_id, proposal_id, cloned_options);

    Some(vote.clone())
}

#[query]
fn get_strategies(space_id: u32) -> Option<Vec<Strategy>> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }

    Some(space.unwrap().strategies.clone())
}

#[query]
fn get_strategy(space_id: u32, strategy_id: u32) -> Option<Strategy> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let strategies = space.unwrap().strategies.clone();
    let strategy = strategies.iter().find(|s| s.id == strategy_id);

    if strategy.is_none() {
        return None;
    }
    Some(strategy.unwrap().clone())
}

#[update]
fn insert_evm_strategy(
    space_id: u32,
    strategy_id: u32,
    name: String,
    description: String,
    evm_strategy: EvmStrategy,
) -> Option<Strategy> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut strategies = space.unwrap().strategies;
    let id = strategies.len() as u32 + 1;
    let new_strategy = types::strategy::Strategy {
        id: id,
        name,
        description,
        space_id,
        evm_strategy: Some(evm_strategy),
    };

    strategies.push(new_strategy.clone());
    update_strategies(space_id, strategies);

    Some(new_strategy)
}

#[update]
fn update_strategies(id: u32, strategies: Vec<Strategy>) {
    let space = get_space(id);
    if space.is_none() {
        return;
    }
    let space = space.unwrap();
    let mut new_space = space.clone();
    new_space.strategies = strategies;

    SPACES.with(|spaces_ref| {
        let mut spaces = spaces_ref.borrow_mut();
        spaces.insert(id, new_space.clone());
    });
}

#[update]
fn update_evm_strategy(
    space_id: u32,
    strategy_id: u32,
    name: String,
    description: String,
    evm_strategy: EvmStrategy,
) -> Option<Strategy> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let mut strategies = space.unwrap().strategies;
    let id = strategies.len() as u32 + 1;
    let new_strategy = types::strategy::Strategy {
        id: id,
        name,
        description,
        space_id,
        evm_strategy: Some(evm_strategy),
    };

    let index = strategies.iter().position(|s| s.id == strategy_id).unwrap();
    strategies[index] = new_strategy.clone();

    update_strategies(space_id, strategies);

    Some(new_strategy)
}

#[update]
fn delete_strategy(space_id: u32, strategy_id: u32) -> Option<Strategy> {
    let space = get_space(space_id);
    if space.is_none() {
        return None;
    }
    let strategies = space.unwrap().strategies.clone();
    let strategy = strategies.iter().find(|s| s.id == strategy_id);

    if strategy.is_none() {
        return None;
    }

    let strategy = strategy.unwrap();
    let index = strategies.iter().position(|p| p.id == strategy_id).unwrap();
    let mut new_strategies = strategies.clone();
    new_strategies.remove(index);
    update_strategies(space_id, new_strategies);

    Some(strategy.clone())
}

ic_cdk::export_candid!();
