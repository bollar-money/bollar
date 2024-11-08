mod exec;
mod init;
mod query;

use cosmwasm_std::{Addr, Api, StdResult};
pub use exec::execute;
pub use init::instantiate;
pub use query::query;

use crate::ContractError;

pub type ContractResult<T> = Result<T, ContractError>;

const CONTRACT_NAME: &str = "crates.io:bollar";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn addr_validate(api: &dyn Api, address: &str) -> StdResult<Addr> {
    api.addr_validate(address)
}
