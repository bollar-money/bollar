use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::HashSet;

/// For Circulating shares
pub const CIRCULATING_SHARES: Item<Uint128> = Item::new("circulating_shares");

pub const EXCHANGE_RATES: Map<String, Uint128> = Map::new("exchange_rates");

#[cw_serde]
pub struct Randomness {
    pub uniform_seed: [u8; 32],
}

#[cw_serde]
#[derive(Default)]
pub struct DeliveryQueue {
    pub receivers: HashSet<Addr>,
}
