use cosmwasm_std::{Addr, Order, Storage};

use crate::{
    models::IntentInfo,
    state::{INTENTS, PENDING_INTENTS},
    StdResult,
};

pub fn all_intents(store: &dyn Storage) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENTS.range(store, None, None, Order::Ascending).collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}

pub fn intents_of_owner(store: &dyn Storage, address: &Addr) -> StdResult<Vec<IntentInfo>> {
    let res: StdResult<Vec<_>> = INTENTS
        .prefix(address)
        .range(store, None, None, Order::Ascending)
        .collect();
    Ok(res?.into_iter().map(|(_, v)| v).collect())
}

pub fn intent_of_owner_contract(store: &dyn Storage, owner: &Addr, contract: &Addr) -> StdResult<Option<IntentInfo>> {
    INTENTS
        .may_load(store, (owner, contract))
}


pub fn save_pending_intent(store: &mut dyn Storage, info: &IntentInfo) -> StdResult<()> {
    PENDING_INTENTS.save(store, info)
}
