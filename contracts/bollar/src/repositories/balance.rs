use cosmwasm_std::{Addr, Storage, Uint128};
use cw20_base::state::BALANCES;

use crate::StdResult;

pub fn query_balance(store: &dyn Storage, address: &Addr) -> StdResult<Uint128> {
    BALANCES
        .may_load(store, address)
        .map(|b| b.unwrap_or_default())
}

// pub fn update_balance(store: &mut dyn Storage, owner: &Addr, amount: Uint128, f: fn(Uint128, Uint128) -> StdResult<Uint128>) -> StdResult<Uint128> {
//     BALANCES.update(
//         store,
//         owner,
//         |balance: Option<Uint128>| -> StdResult<_> {
//             f(balance.unwrap_or_default(), amount)
//             // Ok(balance.unwrap_or_default().checked_sub(amount)?)
//         },
//     )
// }

pub fn sub_balance(store: &mut dyn Storage, owner: &Addr, amount: Uint128) -> StdResult<Uint128> {
    BALANCES.update(store, owner, |balance: Option<Uint128>| -> StdResult<_> {
        Ok(balance.unwrap_or_default().checked_sub(amount)?)
    })
}

pub fn add_balance(store: &mut dyn Storage, owner: &Addr, amount: Uint128) -> StdResult<Uint128> {
    BALANCES.update(store, owner, |balance: Option<Uint128>| -> StdResult<_> {
        Ok(balance.unwrap_or_default() + amount)
    })
}

pub fn sub_sender_and_add_recipient(
    store: &mut dyn Storage,
    sender: &Addr,
    recipient: &Addr,
    amount: Uint128,
) -> StdResult<Uint128> {
    sub_balance(store, sender, amount)?;
    add_balance(store, recipient, amount)
}
