#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Addr, Deps, Env, Order, StdResult};

use crate::msg::QueryMsg;
use crate::repositories::{denom, metadata};
use crate::QueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetMetadata {} => get_metadata(deps),
        QueryMsg::GetDenoms {} => all_denoms(deps),
    }
}

fn get_metadata(deps: Deps) -> StdResult<QueryResponse> {
    metadata::get_from_item(deps.storage).and_then(|resp| to_json_binary(&resp))
}


fn all_denoms(deps: Deps) -> StdResult<QueryResponse> {
    let denoms: StdResult<Vec<_>> = denom::all_denoms(deps.storage);

    denoms.and_then(|res| to_json_binary(&res))
}
