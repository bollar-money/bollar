use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};
use std::collections::HashSet;

use crate::models::{IntentInfo, Metadata};

pub const METADATA: Item<Metadata> = Item::new("metadata");
pub const INTENTS: Map<(&Addr, &Addr), IntentInfo> = Map::new("intents");

#[cw_serde]
pub struct Randomness {
    pub uniform_seed: [u8; 32],
}

#[cw_serde]
#[derive(Default)]
pub struct DeliveryQueue {
    pub receivers: HashSet<Addr>,
}
