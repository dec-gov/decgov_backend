export const idlFactory = ({ IDL }) => {
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
  });
  return IDL.Service({
    'delete_space' : IDL.Func([IDL.Nat32], [IDL.Opt(Space)], []),
    'get_space' : IDL.Func([IDL.Nat32], [IDL.Opt(Space)], ['query']),
    'get_spaces' : IDL.Func([], [IDL.Opt(IDL.Vec(Space))], ['query']),
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
