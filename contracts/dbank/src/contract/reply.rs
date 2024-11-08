use babylon_bindings::BabylonMsg;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{attr, to_json_binary, DepsMut, Env, Reply, Response, StdError, SubMsgResponse};
use cw_utils::parse_instantiate_response_data;
// use cw_utils::parse_instantiate_response_data;

use crate::{
    state::{INTENTS, PENDING_INTENTS},
    ContractError,
};

use super::{InstantiationData, CREATE_INTENT_REPLY_ID};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, reply: Reply) -> Result<Response<BabylonMsg>, ContractError> {
    match reply.id {
        CREATE_INTENT_REPLY_ID => {
            initial_intent_instantiated(deps, env, reply.result.into_result())
        }
        id => Err(ContractError::UnRecognizedReplyId { id }),
    }
}

pub fn initial_intent_instantiated(
    deps: DepsMut,
    _env: Env,
    reply: Result<SubMsgResponse, String>,
) -> Result<Response<BabylonMsg>, ContractError> {
    // Parse data from reply
    let resp = reply.map_err(StdError::generic_err)?;
    let data = resp.data.ok_or(ContractError::ReplyDataMissing {})?;
    let resp = parse_instantiate_response_data(&data)?;

    let intent_addr = &deps.api.addr_validate(&resp.contract_address)?;

    let mut intent_info = PENDING_INTENTS.load(deps.storage)?;
    intent_info.intent_contract = intent_addr.to_owned();

    INTENTS.save(
        deps.storage,
        (&intent_info.creator, &intent_addr),
        &intent_info,
    )?;

    // STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
    //     state.lotteries_count += 1;
    //     Ok(state)
    // })?;

    let attrs = vec![attr("action", "reply_create_intent")];

    let data = InstantiationData {
        addr: intent_addr.to_owned(),
    };

    let data = to_json_binary(&data)?;

    Ok(Response::new().add_attributes(attrs).set_data(data))
}
