use cosmwasm_std::{Storage, Uint128};
use cw20_base::state::{TokenInfo, TOKEN_INFO};
use cw_storage_plus::Item;

use crate::StdResult;

pub fn save(
    item: Item<TokenInfo>,
    store: &mut dyn Storage,
    token_info: &TokenInfo,
) -> StdResult<()> {
    item.save(store, token_info)
}

pub fn save_to_item(store: &mut dyn Storage, token_info: &TokenInfo) -> StdResult<()> {
    save(TOKEN_INFO, store, token_info)
}

pub fn get_from_item(store: &dyn Storage) -> StdResult<TokenInfo> {
    TOKEN_INFO.load(store)
}

pub fn reduce_total_supply(store: &mut dyn Storage, amount: Uint128) -> StdResult<TokenInfo> {
    TOKEN_INFO.update(store, |mut info| -> StdResult<_> {
        info.total_supply = info.total_supply.checked_sub(amount)?;
        Ok(info)
    })
}
