use cosmwasm_std::Storage;
use cw_storage_plus::Item;

use crate::{models::Metadata, msg::MetadataResponse, state::METADATA, StdResult};

pub fn save(item: Item<Metadata>, store: &mut dyn Storage, metadata: &Metadata) -> StdResult<()> {
    item.save(store, metadata)
}

pub fn save_to_item(store: &mut dyn Storage, metadata: &Metadata) -> StdResult<()> {
    save(METADATA, store, metadata)
}

pub fn get_from_item(store: &dyn Storage) -> StdResult<MetadataResponse> {
    METADATA.load(store).map(MetadataResponse::from)
}
