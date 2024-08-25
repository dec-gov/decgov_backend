use candid::CandidType;
use serde::Deserialize;


#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct EvmEvent {
    pub chain_id: u32,
    pub contract_address: String,
    pub config_str: String,
}
