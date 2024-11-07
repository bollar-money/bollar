use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::models::{IntentInfo, Metadata};

pub const METADATA: Item<Metadata> = Item::new("metadata");

pub const DENOMS: Map<String, ()> = Map::new("denoms");
