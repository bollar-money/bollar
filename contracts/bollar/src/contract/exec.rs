use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    attr, Addr, CosmosMsg, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult,
    Uint128, WasmMsg,
};
use cw20::{AllowanceResponse, Cw20ReceiveMsg, Expiration};
use cw20_base::allowances::deduct_allowance;
use cw20_base::state::{MinterData, TokenInfo, ALLOWANCES, ALLOWANCES_SPENDER};

use crate::msg::ExecuteMsg;
use crate::repositories::balance;
use crate::{error::ContractError, repositories::token_info};

use super::{addr_validate, ContractResult};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<BabylonMsg>, ContractError> {
    match msg {
        ExecuteMsg::Mint { recipient, amount } => mint(deps, env, info, recipient, amount),
        ExecuteMsg::Transfer { recipient, amount } => transfer(deps, env, info, recipient, amount),
        ExecuteMsg::TransferFrom {
            owner,
            recipient,
            amount,
        } => transfer_from(deps, env, info, owner, recipient, amount),
        ExecuteMsg::BurnFrom { owner, amount } => burn_from(deps, env, info, owner, amount),
        ExecuteMsg::Burn { amount } => burn(deps, env, info, amount),
        ExecuteMsg::DecreaseAllowance {
            spender,
            amount,
            expires,
        } => decrease_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::IncreaseAllowance {
            spender,
            amount,
            expires,
        } => increase_allowance(deps, env, info, spender, amount, expires),
        ExecuteMsg::Send {
            contract,
            amount,
            msg,
        } => send(deps, env, info, contract, amount, msg),
        ExecuteMsg::SendFrom {
            owner,
            contract,
            amount,
            msg,
        } => send_from(deps, env, info, owner, contract, amount, msg),
        // ExecuteMsg::Receive(msg) => receive(deps, info, msg),
    }
}

pub fn mint(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> Result<Response<BabylonMsg>, ContractError> {
    check_zero(amount)?;

    let mut ti: TokenInfo = token_info::get_from_item(deps.storage)?;

    check_total_supply(&ti)?;
    check_minter(&ti, &info.sender)?;

    // update supply and enforce cap
    ti.total_supply += amount;

    token_info::save_to_item(deps.storage, &ti)?;

    // add amount to recipient balance
    let rcpt_addr = deps.api.addr_validate(&recipient)?;

    balance::add_balance(deps.storage, &rcpt_addr, amount)?;

    let res = Response::new()
        .add_attribute("action", "mint")
        .add_attribute("to", recipient)
        .add_attribute("amount", amount);
    Ok(res)
}

pub fn transfer(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    check_zero(amount)?;

    let recipient = deps.api.addr_validate(&recipient)?;

    // Subtract the sender amount and add the recipient amount
    balance::sub_sender_and_add_recipient(deps.storage, &info.sender, &recipient, amount)?;

    let resp = Response::new()
        .add_attribute("action", "transfer")
        .add_attribute("from", &info.sender)
        .add_attribute("to", &recipient)
        .add_attribute("amount", amount);

    Ok(resp)
}

pub fn transfer_from(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    owner: String,
    recipient: String,
    amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    check_zero(amount)?;
    let recipient = addr_validate(deps.api, &recipient)?;
    let owner = addr_validate(deps.api, &owner)?;

    // Deduct allowance before transfer
    deduct_allowance(deps.storage, &owner, &info.sender, &env.block, amount)?;

    // Subtract the sender amount and add the recipient amount
    balance::sub_sender_and_add_recipient(deps.storage, &owner, &recipient, amount)?;

    let res = Response::new().add_attributes(vec![
        attr("action", "transfer_from"),
        attr("from", owner),
        attr("to", recipient),
        attr("by", info.sender),
        attr("amount", amount),
    ]);

    Ok(res)
}

pub fn burn(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    check_zero(amount)?;

    // Subtract the balance of sender
    balance::sub_balance(deps.storage, &info.sender, amount)?;

    // reduce the total supply
    token_info::reduce_total_supply(deps.storage, amount)?;

    let res = Response::new()
        .add_attribute("action", "burn")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount);

    Ok(res)
}

pub fn burn_from(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    owner: String,
    amount: Uint128,
) -> ContractResult<Response<BabylonMsg>> {
    check_zero(amount)?;

    let owner = addr_validate(deps.api, &owner)?;

    // Deduct allowance before transfer
    deduct_allowance(deps.storage, &owner, &info.sender, &env.block, amount)?;

    // Subtract the balance of sender
    balance::sub_balance(deps.storage, &info.sender, amount)?;

    // reduce the total supply
    token_info::reduce_total_supply(deps.storage, amount)?;

    let res = Response::new()
        .add_attribute("action", "burn")
        .add_attribute("from", info.sender)
        .add_attribute("amount", amount);

    Ok(res)
}

pub fn increase_allowance(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response<BabylonMsg>, ContractError> {
    let spender_addr = deps.api.addr_validate(&spender)?;
    if spender_addr == info.sender {
        return Err(ContractError::CannotSetOwnAccount {});
    }

    let update_fn = |allow: Option<AllowanceResponse>| -> Result<_, _> {
        let mut val = allow.unwrap_or_default();
        if let Some(exp) = expires {
            if exp.is_expired(&env.block) {
                return Err(ContractError::InvalidExpiration {});
            }
            val.expires = exp;
        }
        val.allowance += amount;
        Ok(val)
    };
    ALLOWANCES.update(deps.storage, (&info.sender, &spender_addr), update_fn)?;
    ALLOWANCES_SPENDER.update(deps.storage, (&spender_addr, &info.sender), update_fn)?;

    let res = Response::new().add_attributes(vec![
        attr("action", "increase_allowance"),
        attr("owner", info.sender),
        attr("spender", spender),
        attr("amount", amount),
    ]);
    Ok(res)
}

pub fn decrease_allowance(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    spender: String,
    amount: Uint128,
    expires: Option<Expiration>,
) -> Result<Response<BabylonMsg>, ContractError> {
    let spender_addr = addr_validate(deps.api, &spender)?;

    if spender_addr == info.sender {
        return Err(ContractError::CannotSetOwnAccount {});
    }

    let key = (&info.sender, &spender_addr);

    fn reverse<'a>(t: (&'a Addr, &'a Addr)) -> (&'a Addr, &'a Addr) {
        (t.1, t.0)
    }

    // load value and delete if it hits 0, or update otherwise
    let mut allowance = ALLOWANCES.load(deps.storage, key)?;
    if amount < allowance.allowance {
        // update the new amount
        allowance.allowance = allowance
            .allowance
            .checked_sub(amount)
            .map_err(StdError::overflow)?;
        if let Some(exp) = expires {
            if exp.is_expired(&env.block) {
                return Err(ContractError::InvalidExpiration {});
            }
            allowance.expires = exp;
        }
        ALLOWANCES.save(deps.storage, key, &allowance)?;
        ALLOWANCES_SPENDER.save(deps.storage, reverse(key), &allowance)?;
    } else {
        ALLOWANCES.remove(deps.storage, key);
        ALLOWANCES_SPENDER.remove(deps.storage, reverse(key));
    }

    let res = Response::new().add_attributes(vec![
        attr("action", "decrease_allowance"),
        attr("owner", info.sender),
        attr("spender", spender),
        attr("amount", amount),
    ]);

    Ok(res)
}

pub fn send(
    deps: DepsMut<BabylonQuery>,
    _env: Env,
    info: MessageInfo,
    contract: String,
    amount: Uint128,
    msg: QueryResponse,
) -> ContractResult<Response<BabylonMsg>> {
    check_zero(amount)?;

    let recipient = deps.api.addr_validate(&contract)?;

    // Subtract the sender amount and add the recipient amount
    balance::sub_sender_and_add_recipient(deps.storage, &info.sender, &recipient, amount)?;

    let resp = Response::<BabylonMsg>::new()
        .add_attribute("action", "send")
        .add_attribute("from", &info.sender)
        .add_attribute("to", &contract)
        .add_attribute("amount", amount)
        .add_message(into_cosmos_msg(
            Cw20ReceiveMsg {
                sender: info.sender.into(),
                amount,
                msg,
            },
            contract,
        )?);

    Ok(resp)
}

pub fn send_from(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    owner: String,
    contract: String,
    amount: Uint128,
    msg: QueryResponse,
) -> Result<Response<BabylonMsg>, ContractError> {
    let recipient = deps.api.addr_validate(&contract)?;
    let owner = deps.api.addr_validate(&owner)?;

    // deduct allowance before doing anything else have enough allowance
    deduct_allowance(deps.storage, &owner, &info.sender, &env.block, amount)?;

    // Subtract the sender amount and add the recipient amount
    balance::sub_sender_and_add_recipient(deps.storage, &owner, &recipient, amount)?;

    let attrs = vec![
        attr("action", "send_from"),
        attr("from", &owner),
        attr("to", &contract),
        attr("by", &info.sender),
        attr("amount", amount),
    ];

    // create a send message
    let msg = into_cosmos_msg(
        Cw20ReceiveMsg {
            sender: info.sender.into(),
            amount,
            msg,
        },
        contract,
    )?;

    let res = Response::<BabylonMsg>::new()
        .add_message(msg)
        .add_attributes(attrs);

    Ok(res)
}

// pub fn receive(
//     deps: DepsMut,
//     info: MessageInfo,
//     wrapper: Cw20ReceiveMsg,
// ) -> Result<Response, ContractError> {
//     let msg: ReceiveMsg = from_json(&wrapper.msg)?;
//     let balance = Balance::Cw20(Cw20CoinVerified {
//         address: info.sender,
//         amount: wrapper.amount,
//     });
//     let api = deps.api;
//     match msg {
//         ReceiveMsg::Create(msg) => {
//             execute_create(deps, msg, balance, &api.addr_validate(&wrapper.sender)?)
//         }
//         ReceiveMsg::TopUp { id } => execute_top_up(deps, id, balance),
//     }
// }

pub fn check_zero(amount: Uint128) -> ContractResult<()> {
    if amount == Uint128::zero() {
        Err(ContractError::InvalidZeroAmount {})
    } else {
        Ok(())
    }
}

pub fn check_minter(token_info: &TokenInfo, sender: &Addr) -> ContractResult<()> {
    if let Some(MinterData { minter, .. }) = token_info.mint.clone() {
        println!("Minter is: {minter:?}, sender is: {sender:?}");

        if minter == sender {
            return Ok(());
        }
    }

    Err(ContractError::Unauthorized {})
}

pub fn check_total_supply(token_info: &TokenInfo) -> ContractResult<()> {
    if let Some(limit) = token_info.get_cap() {
        if token_info.total_supply > limit {
            return Err(ContractError::CannotExceedCap {});
        }
    }

    Ok(())
}

/// creates a cosmos_msg sending this struct to the named contract
/// Fix the cw20-base `into_cosmos_msg` function in Cw20ReceiveMsg
pub fn into_cosmos_msg<T>(msg: Cw20ReceiveMsg, contract_addr: String) -> StdResult<CosmosMsg<T>> {
    let msg = msg.into_json_binary()?;
    let execute = WasmMsg::Execute {
        contract_addr,
        msg,
        funds: vec![],
    };
    Ok(execute.into())
}
