use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub struct Metadata {
    // pub staked_token_denom: String,
    // pub staked_token_amount: Uint128,
    // pub leverage: u8,
    pub bollar_vault: Addr,
    pub creator: Addr,
    pub created_at: Timestamp,
    pub status: DBankStatus,
}

#[cw_serde]
pub enum DBankStatus {
    Activing,
    Stopped,
}
