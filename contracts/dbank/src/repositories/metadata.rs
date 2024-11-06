use cosmwasm_std::Storage;
use cw_storage_plus::Item;

use crate::{models::Metadata, msg::MetadataResp, state::METADATA, StdResult};

pub fn save(item: Item<Metadata>, store: &mut dyn Storage, token_info: &Metadata) -> StdResult<()> {
    item.save(store, token_info)
}

pub fn save_to_item(store: &mut dyn Storage, token_info: &Metadata) -> StdResult<()> {
    save(METADATA, store, token_info)
}

pub fn get_from_item(store: &dyn Storage) -> StdResult<MetadataResp> {
    METADATA.load(store).map(MetadataResp::from)
}
