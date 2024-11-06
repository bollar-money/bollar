use cosmwasm_std::{Addr, Storage, Uint128};
use cw20_base::state::BALANCES;

use crate::StdResult;

// pub fn save(map: Map<Addr, Uint128>, store: &mut dyn Storage, key: Addr, balance: &Uint128) -> StdResult<()> {
//     map.save(store, key, balance)
// }

// pub fn save_to_item(store: &mut dyn Storage, key: Addr, balance: &Uint128) -> StdResult<()> {
//     save(BALANCE_OF_ADDRESS, store, key, balance)
// }

// pub fn get_balance_of(store: &dyn Storage, key: Addr) -> StdResult<BalanceOfResp> {
//     BALANCE_OF_ADDRESS.load(store, key)
//         .map(BalanceOfResp::from)
// }

pub fn query_balance(store: &dyn Storage, address: &Addr) -> StdResult<Uint128> {
    let balance = BALANCES.load(store, address);
    
    if balance.is_err() { return Ok(Uint128::zero()); }

    balance
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
