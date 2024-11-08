use cosmwasm_std::{Addr, Order, Storage};

use crate::{models::IntentInfo, state::INTENT_LEVERAGES, StdResult};

pub fn all_intents(store: &dyn Storage) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENT_LEVERAGES
        .range(store, None, None, Order::Ascending)
        .collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}

pub fn intents_of(store: &dyn Storage, address: &Addr) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENT_LEVERAGES
        .sub_prefix(address)
        .range(store, None, None, Order::Ascending)
        .collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}

pub fn save(
    store: &mut dyn Storage,
    owner: &Addr,
    leverage: u8,
    denom: String,
    info: &IntentInfo,
) -> StdResult<()> {
    INTENT_LEVERAGES.save(store, (owner, leverage, denom), info)
}
