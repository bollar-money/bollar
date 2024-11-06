use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
use cw20_base::state::{MinterData, TokenInfo};

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::repositories::token_info;

use super::{CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<BabylonMsg>, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // store token info using cw20-base format
    let token_info = TokenInfo {
        name: msg.name,
        symbol: msg.symbol.clone(),
        decimals: msg.decimals,
        total_supply: Uint128::zero(),
        mint: Some(MinterData {
            minter: info.sender.clone(),
            cap: None,
        }),
    };

    token_info::save_to_item(deps.storage, &token_info)?;

    Ok(Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute("symbol", msg.symbol)
        .add_attribute("created_at", env.block.time.to_string())
        .add_attribute("creator", info.sender))
}
