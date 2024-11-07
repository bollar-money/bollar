
use cw_storage_plus::Item;

use crate::models::Metadata;

pub const METADATA: Item<Metadata> = Item::new("metadata");


