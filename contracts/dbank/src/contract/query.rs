
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;
use crate::repositories::{denom, intent, metadata};
use crate::QueryResponse;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetMetadata {} => get_metadata(deps),
        QueryMsg::AllIntnet {} => all_intents(deps),
        QueryMsg::IntentsOfOwner { owner } => intents_of(deps, owner),
        QueryMsg::IntentOfOwnerContract { owner, contract } => intent_by_owner_contract(deps, owner, contract),
        QueryMsg::GetDenoms {} => all_denoms(deps),
    }
}

fn get_metadata(deps: Deps) -> StdResult<QueryResponse> {
    metadata::get_from_item(deps.storage).and_then(|resp| to_json_binary(&resp))
}

fn all_intents(deps: Deps) -> StdResult<QueryResponse> {
    let intents: StdResult<Vec<_>> = intent::all_intents(deps.storage);

    intents.and_then(|i| to_json_binary(&i))
}

fn intents_of(deps: Deps, address: String) -> StdResult<QueryResponse> {
    let address = deps.api.addr_validate(&address)?;
    let intents: StdResult<Vec<_>> = intent::intents_of_owner(deps.storage, &address);

    intents.and_then(|i| to_json_binary(&i))
}

fn intent_by_owner_contract(deps: Deps, owner: String, contract: String) -> StdResult<QueryResponse> {
    let owner = deps.api.addr_validate(&owner)?;
    let contract = deps.api.addr_validate(&contract)?;
    let intents: StdResult<Option<_>> = intent::intent_of_owner_contract(deps.storage, &owner, &contract);

    intents.and_then(|i| to_json_binary(&i))
}
fn all_denoms(deps: Deps) -> StdResult<QueryResponse> {
    let denoms: StdResult<Vec<_>> = denom::all_denoms(deps.storage);

    denoms.and_then(|res| to_json_binary(&res))
}
