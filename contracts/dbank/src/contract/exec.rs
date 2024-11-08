use babylon_bindings::{BabylonMsg, BabylonQuery};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, to_json_binary, Addr, DepsMut, Env, MessageInfo, Response, SubMsg, WasmMsg,
};

use crate::{
    models::{IntentInfo, IntentInstantiateMsg},
    msg::ExecuteMsg,
    repositories::{self, denom, intent_leverage, metadata},
    ContractError,
};

use super::{ContractResult, CREATE_INTENT_REPLY_ID};

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

    if coins.len() != 1 {
        return Err(ContractError::UnsupportedMultiDenom {});
    }

    let coin = coins[0].clone();

    if coin.amount.u128() == 0 {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let denom = coin.denom;
    let sender = info.sender;
    let denoms = denom::all_denoms(deps.storage)?;

    if !denoms.contains(&denom) {
        return Err(ContractError::InvalidDenomStaking { denom });
    }

    // Create Intent contract
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
        intent_contract: Addr::unchecked(""),
        bollar_vault: metadata.bollar_vault,
        leverage,
        creator: sender.clone(),
        created_at: env.block.time,
    };

    repositories::intent::save_pending_intent(deps.storage, &intent_info)?;
    intent_leverage::save(deps.storage, &sender, leverage, denom, &intent_info)?;

    // handle reply
    let msg = SubMsg::reply_on_success(msg, CREATE_INTENT_REPLY_ID);

    let attrs = vec![
        attr("action", "create_intent"),
        attr("sender", sender),
        attr("intent_name", name),
        attr("intent_leverage", leverage.to_string()),
    ];

    let resp = Response::new().add_submessage(msg).add_attributes(attrs);

    Ok(resp)
}
