use cosmwasm_schema::cw_serde;
use cosmwasm_std::{QueryResponse, Uint128};
use cw20::Expiration;

#[cw_serde]
pub enum ExecuteMsg {
    // Stake BBN to create Intent contract
    Stake { amount: Uint128 },
}
