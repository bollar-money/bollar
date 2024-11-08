mod exec;
mod init;
mod query;
mod reply;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, StdResult};

pub use exec::execute;
pub use init::instantiate;
pub use query::query;
pub use reply::reply;

use crate::ContractError;

pub type ContractResult<T> = Result<T, ContractError>;

const CONTRACT_NAME: &str = "crates.io:bollar";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const CREATE_INTENT_REPLY_ID: u64 = 1;

pub fn addr_validate(api: &dyn Api, address: &str) -> StdResult<Addr> {
    api.addr_validate(address)
}

#[cw_serde]
pub struct InstantiationData {
    pub addr: Addr,
}
