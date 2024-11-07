use std::collections::HashSet;

use babylon_bindings::{BabylonMsg, BabylonQuery};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, QueryResponse,
    Response, StdError, StdResult, SubMsg, Uint128, WasmMsg,
};

use crate::{
    models::{IntentInfo, IntentInstantiateMsg},
    msg::ExecuteMsg,
    repositories::{denom, metadata},
    ContractError,
};

use super::{addr_validate, ContractResult, CREATE_INTENT_REPLY_ID};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response<BabylonMsg>> {
    match msg {
        ExecuteMsg::Stake { leverage, name } => stake(deps, env, info, leverage, name),
    }
}

fn stake(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    leverage: u8,
    name: String,
) -> ContractResult<Response<BabylonMsg>> {
    // Get BBN funds
    let coins = info.funds;

    if coins.iter().map(|coin| coin.amount.u128()).sum::<u128>() == 0 {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let sender = info.sender;
    let denoms = denom::all_denoms(deps.storage)?;

    let denom_set: HashSet<_> = denoms.iter().cloned().collect();

    for coin in &coins {
        if !denom_set.contains(&coin.denom) {
            return Err(ContractError::InvalidDenomStaking {
                denom: coin.denom.clone(),
            });
        }
    }

    // TODO: Create Intent contract
    let metadata = metadata::get_from_item(deps.storage)?.metadata;
    let intent_code_id = metadata.intent_code_id;

    let intent_instantiate_msg = IntentInstantiateMsg {
        name: name.clone(),
        leverage,
        bollar_vault: metadata.bollar_vault.to_string(),
    };

    let msg = WasmMsg::Instantiate {
        admin: Some(sender.clone().to_string()),
        code_id: intent_code_id,
        msg: to_json_binary(&intent_instantiate_msg)?,
        funds: coins,
        label: name.clone(),
    };

    let intent_info = IntentInfo {
        name: name.clone(),
        bollar_vault: metadata.bollar_vault,
        leverage,
        creator: sender.clone(),
        created_at: env.block.time,
    };

    // intent::
    // TODO: handle reply
    let msg = SubMsg::reply_never(msg);

    let attrs = vec![
        attr("action", "create_intent"),
        attr("sender", sender),
        attr("intent_name", name),
        attr("intent_leverage", leverage.to_string()),
    ];

    let resp = Response::new().add_submessage(msg).add_attributes(attrs);

    Ok(resp)
}
