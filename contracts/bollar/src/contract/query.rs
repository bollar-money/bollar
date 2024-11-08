#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Deps, Env, StdResult, Uint128};
use cw20_base::allowances;

use crate::msg::{QueryMsg, TotalSupplyCirculatingResponse};
use crate::repositories::{balance, circulating, exchanger, token_info};
use crate::QueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::TokenInfo {} => query_token_info(deps),
        QueryMsg::Balance { address } => query_balance(deps, &address),
        QueryMsg::Allowance { owner, spender } => query_allowance(deps, owner, spender),
        QueryMsg::TotalSupplyCirculating {} => query_total_supply_circulating_(deps),
        QueryMsg::GetExchangeRate { denom } => query_exchange_rate(deps, denom),
    }
}

pub fn query_token_info(deps: Deps) -> StdResult<QueryResponse> {
    token_info::get_from_item(deps.storage).and_then(|info| to_json_binary(&info))
}

pub fn query_balance(deps: Deps, address: &str) -> StdResult<QueryResponse> {
    let address = deps.api.addr_validate(address)?;
    balance::query_balance(deps.storage, &address).and_then(uint128_to_binary)
}

pub fn query_allowance(deps: Deps, owner: String, spender: String) -> StdResult<QueryResponse> {
    allowances::query_allowance(deps, owner, spender).and_then(|ar| to_json_binary(&ar))
}

pub fn query_total_supply_circulating_(deps: Deps) -> StdResult<QueryResponse> {
    let total_supply = token_info::get_from_item(deps.storage).map(|t| t.total_supply)?;
    let circulating_shares = circulating::get_from_item(deps.storage)?;

    let resp = TotalSupplyCirculatingResponse {
        total_supply,
        circulating_shares,
    };

    to_json_binary(&resp)
}

pub fn query_exchange_rate(deps: Deps, denom: String) -> StdResult<QueryResponse> {
    exchanger::get(deps.storage, denom).and_then(uint128_to_binary)
}

fn uint128_to_binary(v: Uint128) -> StdResult<QueryResponse> {
    to_json_binary(&v)
}
