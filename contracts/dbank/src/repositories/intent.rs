use cosmwasm_std::{Addr, Order, Storage};

use crate::{models::IntentInfo, state::INTENTS, StdResult};

pub fn all_intents(
    store: &dyn Storage,
) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENTS.range(store, None, None, Order::Ascending).collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}

pub fn intents_of(
    store: &dyn Storage,
    address: &Addr
) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENTS.prefix(address).range(store, None, None, Order::Ascending).collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}
