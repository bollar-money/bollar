use cosmwasm_std::{Addr, Storage};
use cw20::AllowanceResponse;
use cw20_base::state::ALLOWANCES;

use crate::StdResult;

pub fn query_allowance(
    store: &dyn Storage,
    owner: &Addr,
    spender: &Addr,
) -> StdResult<AllowanceResponse> {
    ALLOWANCES
        .may_load(store, (owner, spender))
        .map(|a| a.unwrap_or_default())
}
