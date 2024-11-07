use cosmwasm_std::{Storage, Uint128};
use cw_storage_plus::Item;

use crate::{state::CIRCULATING_SHARES, StdResult};

pub fn save(
    item: Item<Uint128>,
    storage: &mut dyn Storage,
    circulating_share: &Uint128,
) -> StdResult<()> {
    item.save(storage, circulating_share)
}

pub fn save_to_item(storage: &mut dyn Storage, circulating_share: &Uint128) -> StdResult<()> {
    save(CIRCULATING_SHARES, storage, circulating_share)
}

pub fn get_from_item(storage: &dyn Storage) -> StdResult<Uint128> {
    CIRCULATING_SHARES.may_load(storage).map(|c| c.unwrap_or_default())
}
