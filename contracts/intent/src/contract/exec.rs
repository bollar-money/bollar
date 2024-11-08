use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::{msg::ExecuteMsg, ContractError};

use super::ContractResult;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response<BabylonMsg>> {
    match msg {
        ExecuteMsg::Stake {} => stake(deps, env, info),
    }
}

fn stake(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    // amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    // Get BBN funds
    let coins = info.funds;

    if coins.iter().map(|coin| coin.amount.u128()).sum::<u128>() == 0 {
        return Err(ContractError::InvalidZeroAmount {});
    }

    // save metadata TODO:

    let resp = Response::new()
        // .add_message(message)
        .add_attribute("action", "stake");

    Ok(resp)
}
