use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::models::{IntentInfo, Metadata};

pub const METADATA: Item<Metadata> = Item::new("metadata");

pub const INTENTS: Map<(&Addr, &Addr), IntentInfo> = Map::new("intents");
pub const INTENT_LEVERAGES: Map<(&Addr, u8, String), IntentInfo> =
    Map::new("intent_leverage_denom");

/// Cache Intent info
pub const PENDING_INTENTS: Item<IntentInfo> = Item::new("pending_intents");

pub const DENOMS: Map<String, ()> = Map::new("denoms");
