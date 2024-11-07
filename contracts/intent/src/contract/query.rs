#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;
use crate::repositories::metadata;
use crate::QueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetMetadata {} => get_metadata(deps),
    }
}

fn get_metadata(deps: Deps) -> StdResult<QueryResponse> {
    metadata::get_from_item(deps.storage).and_then(|resp| to_json_binary(&resp))
}
