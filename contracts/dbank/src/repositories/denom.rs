use cosmwasm_std::{Order, Storage};

use crate::{state::DENOMS, StdResult};

pub fn all_denoms(store: &dyn Storage) -> StdResult<Vec<String>> {
    DENOMS.keys(store, None, None, Order::Ascending).collect()
}

pub fn exists(store: &dyn Storage, denom: String) -> bool {
    DENOMS.has(store, denom)
}

pub fn save(store: &mut dyn Storage, denom: String) -> StdResult<()> {
    DENOMS.save(store, denom, &())
}
