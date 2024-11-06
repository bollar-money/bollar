use std::collections::HashSet;

use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult, Uint128, WasmMsg
};

use crate::{msg::ExecuteMsg, repositories::denom, ContractError};

use super::{addr_validate, ContractResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response<BabylonMsg>> {
    match msg {
        ExecuteMsg::Stake {} => todo!(),
    }
}

fn stake(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    // Get BBN funds
    let coins = info.funds;

    if coins.iter().map(|coin| coin.amount.u128()).sum::<u128>() == 0 {
        return Err(ContractError::InvalidZeroAmount {});
    }

    let denoms = denom::all_denoms(deps.storage)?;

    let denom_set: HashSet<_> = denoms.iter().cloned().collect();

    for coin in &coins {
        if !denom_set.contains(&coin.denom) {
            return Err(ContractError::InvalidDenomStaking { denom: coin.denom.clone() });
        }
    }

    // transfer funds to dbank,
    let message = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: coins,
    };

    // TODO: Create Intent contract

    let resp = Response::new()
            .add_message(message)
            .add_attribute("action", "donate");

    Ok(resp)
}
