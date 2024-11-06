use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult,
    Uint128, WasmMsg,
};


use crate::msg::ExecuteMsg;

use crate::error::ContractError;

use super::{addr_validate, ContractResult};
// use crate::repositories::{metadata, total_supply};
// use crate::state::{BALANCE_OF_ADDRESS, BEACONS, TOKEN_INFO, TOTAL_SUPPLY};
// use crate::types::Metadata;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<BabylonMsg>, ContractError> {
   
    
    todo!()
}

