use cw_storage_plus::Item;

use crate::models::{IntentOperation, Metadata};

pub const METADATA: Item<Metadata> = Item::new("metadata");

/// Intent record
pub const INTENTS: Item<Vec<IntentOperation>> = Item::new("intents");
