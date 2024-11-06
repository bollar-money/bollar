use cosmwasm_std::{Storage, Uint128};
use cw_storage_plus::Item;

use crate::{msg::TotalSupplyResp, state::TOTAL_SUPPLY, StdResult};

pub fn save(
    item: Item<Uint128>,
    storage: &mut dyn Storage,
    total_supply: &Uint128,
) -> StdResult<()> {
    item.save(storage, total_supply)
}

pub fn save_to_item(storage: &mut dyn Storage, total_supply: &Uint128) -> StdResult<()> {
    save(TOTAL_SUPPLY, storage, total_supply)
}

pub fn get_from_item(storage: &dyn Storage) -> StdResult<TotalSupplyResp> {
    TOTAL_SUPPLY.load(storage).map(TotalSupplyResp::from)
}
