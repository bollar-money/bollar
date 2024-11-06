use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use std::collections::HashSet;

use crate::models::Metadata;

/// For Total supply
pub const TOTAL_SUPPLY: Item<Uint128> = Item::new("total_supply");

/// Balance of address
pub const BALANCE_OF_ADDRESS: Map<Addr, Uint128> = Map::new("balance_of_address");

/// For Token Info
pub const METADATA: Item<Metadata> = Item::new("token_info");

pub const BEACONS: Map<u64, Randomness> = Map::new("beacons");
pub const DELIVERY_QUEUES: Map<u64, DeliveryQueue> = Map::new("delivery_queues");

#[cw_serde]
pub struct Randomness {
    pub uniform_seed: [u8; 32],
}

#[cw_serde]
#[derive(Default)]
pub struct DeliveryQueue {
    pub receivers: HashSet<Addr>,
}
