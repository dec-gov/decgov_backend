export const idlFactory = ({ IDL }) => {
  const ProposalOptionVote = IDL.Record({
    'id' : IDL.Nat32,
    'signature' : IDL.Text,
    'vote_type' : IDL.Nat32,
    'option_id' : IDL.Nat32,
    'user_address' : IDL.Text,
    'timestamp' : IDL.Nat32,
    'voting_power' : IDL.Nat64,
  });
  const ProposalOption = IDL.Record({
    'id' : IDL.Nat32,
    'votes' : IDL.Vec(ProposalOptionVote),
    'name' : IDL.Text,
    'on_win_contract_address' : IDL.Text,
    'proposal_id' : IDL.Nat32,
    'on_win_bytecode' : IDL.Text,
    'on_win_chain_id' : IDL.Nat32,
  });
  const Proposal = IDL.Record({
    'id' : IDL.Nat32,
    'title' : IDL.Text,
    'date_created' : IDL.Nat32,
    'mechanism' : IDL.Nat32,
    'description' : IDL.Text,
    'options' : IDL.Vec(ProposalOption),
    'space_id' : IDL.Nat32,
  });
  const Space = IDL.Record({
    'id' : IDL.Nat32,
    'vote_delay' : IDL.Nat32,
    'vote_duration' : IDL.Nat32,
    'name' : IDL.Text,
    'website_link' : IDL.Text,
    'icon_link' : IDL.Text,
    'min_vote_role' : IDL.Nat32,
    'min_vote_power' : IDL.Nat64,
    'quorum' : IDL.Nat32,
    'options' : IDL.Vec(Proposal),
  });
  return IDL.Service({
    'delete_space' : IDL.Func([IDL.Nat32], [IDL.Opt(Space)], []),
    'get_space' : IDL.Func([IDL.Nat32], [IDL.Opt(Space)], ['query']),
    'get_spaces' : IDL.Func([], [IDL.Opt(IDL.Vec(Space))], ['query']),
    'insert_proposal' : IDL.Func(
        [IDL.Nat32, IDL.Text, IDL.Text, IDL.Nat32, IDL.Nat32],
        [IDL.Opt(Proposal)],
        [],
      ),
    'insert_space' : IDL.Func(
        [
          IDL.Text,
          IDL.Text,
          IDL.Text,
          IDL.Nat32,
          IDL.Nat32,
          IDL.Nat32,
          IDL.Nat64,
          IDL.Nat32,
        ],
        [Space],
        [],
      ),
    'update_space' : IDL.Func(
        [
          IDL.Nat32,
          IDL.Text,
          IDL.Text,
          IDL.Text,
          IDL.Nat32,
          IDL.Nat32,
          IDL.Nat32,
          IDL.Nat64,
          IDL.Nat32,
        ],
        [IDL.Opt(Space)],
        [],
      ),
  });
};
export const init = ({ IDL }) => { return []; };
