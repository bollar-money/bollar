use std::collections::HashMap;

use cosmwasm_std::{Storage, Uint128};

use crate::{state::EXCHANGE_RATES, StdResult};

pub fn get(store: &dyn Storage, denom: String) -> StdResult<Uint128> {
    EXCHANGE_RATES.load(store, denom)
}

pub fn all(store: &dyn Storage) -> StdResult<HashMap<String, Uint128>> {
    EXCHANGE_RATES
        .range(store, None, None, cosmwasm_std::Order::Ascending)
        .collect()
}

pub fn save(store: &mut dyn Storage, denom: String, rate: Uint128) -> StdResult<()> {
    EXCHANGE_RATES.save(store, denom, &rate)
}
