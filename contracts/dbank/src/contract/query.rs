#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Deps, Env, StdResult};
use cw20_base::allowances;

use crate::msg::QueryMsg;
use crate::repositories::{balance, token_info};
use crate::QueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    // match msg {
    //     QueryMsg::TokenInfo {} => query_token_info(deps),
    //     QueryMsg::Balance { address } => query_balance(deps, &address),
    //     QueryMsg::Allowance { owner, spender } => query_allowance(deps, owner, spender),
    // }
    todo!()
}

pub fn query_token_info(deps: Deps) -> StdResult<QueryResponse> {
    token_info::get_from_item(deps.storage).and_then(|info| to_json_binary(&info))
}

pub fn query_balance(deps: Deps, address: &str) -> StdResult<QueryResponse> {
    let address = deps.api.addr_validate(address)?;
    balance::query_balance(deps.storage, &address).and_then(|b| to_json_binary(&b))
}

pub fn query_allowance(deps: Deps, owner: String, spender: String) -> StdResult<QueryResponse> {
    allowances::query_allowance(deps, owner, spender).and_then(|ar| to_json_binary(&ar))
}
