use babylon_bindings::{BabylonMsg, BabylonQuery};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::models::Metadata;
use crate::msg::InstantiateMsg;
use crate::repositories;
use crate::{error::ContractError, models::IntentStatus};

use super::{CONTRACT_NAME, CONTRACT_VERSION};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<BabylonQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<BabylonMsg>, ContractError> {
    cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // let metadata = Metadata {
    //     bollar_vault: deps.api.addr_validate(&msg.bollar_vault)?,
    //     creator: info.sender,
    //     created_at: env.block.time,
    //     status: IntentStatus::Activing,
    // };

    // repositories::metadata::save_to_item(deps.storage, &metadata)?;

    // for denom in msg.denoms {
    //     repositories::denom::save(deps.storage, denom)?;
    // }

    Ok(Response::new())
}
