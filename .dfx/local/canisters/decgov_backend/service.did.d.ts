import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface Space {
  'id' : number,
  'vote_delay' : number,
  'vote_duration' : number,
  'name' : string,
  'website_link' : string,
  'icon_link' : string,
  'min_vote_role' : number,
  'min_vote_power' : bigint,
  'quorum' : number,
}
export interface _SERVICE {
  'delete_space' : ActorMethod<[number], [] | [Space]>,
  'get_space' : ActorMethod<[number], [] | [Space]>,
  'get_spaces' : ActorMethod<[], Array<Space>>,
  'insert_space' : ActorMethod<
    [string, string, string, number, number, number, bigint, number],
    Space
  >,
  'update_space' : ActorMethod<
    [number, string, string, string, number, number, number, bigint, number],
    [] | [Space]
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
