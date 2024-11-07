use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use cw20::{Cw20Coin, MinterResponse};
use cw20_base::msg::{InstantiateMarketingInfo, InstantiateMsg as Cw20InstantiateMsgstate};

use crate::error::ContractError;
use crate::msg::InstantiateMsg;

use super::{CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<BabylonMsg>, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let description = "`Bollar` - Bitcoin dollar, inspired by `bitcoin`, follows bitcoin's money theory and combines the PoS chain's revenue model, deployed on the `Babylon` chain. `Bollar` is a stablecoin, Bollar is issued by the `Bollar protocol`";

    let name = msg.name;
    let symbol = msg.symbol;
    let decimals = msg.decimals;
    let created_at = env.block.time.to_string();
    let sender = info.sender.clone();
    let amount = msg.amount;

    let msg = Cw20InstantiateMsgstate {
        name: name.clone(),
        symbol: symbol.clone(),
        decimals,
        initial_balances: vec![Cw20Coin {
            address: sender.clone().to_string(),
            amount,
        }],
        mint: Some(MinterResponse {
            minter: sender.to_string(),
            cap: None,
        }),
        marketing: Some(InstantiateMarketingInfo {
            project: Some("Bollar Money".to_string()),
            description: Some(description.to_string()),
            marketing: None,
            logo: None,
        }),
    };

    cw20_base::contract::instantiate(deps, env.clone(), info.clone(), msg)?;

    Ok(Response::default()
        .add_attribute("method", "instantiate")
        .add_attribute("symbol", symbol)
        .add_attribute("created_at", env.block.time.to_string())
        .add_attribute("creator", info.sender)
        .add_attribute("created_at", created_at.to_string()))
}
