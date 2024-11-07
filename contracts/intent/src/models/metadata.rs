use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Timestamp, Uint128};

/// Intent contract metadata should keep the creator, staked assets, status,
#[cw_serde]
pub struct Metadata {
    pub staked_coin: Coin,
    pub leverage: u8,
    pub bollar_vault: Addr,
    pub interest_to_pay: Uint128,
    pub fee: u64,
    pub creator: Addr,
    pub created_at: Timestamp,
    pub status: IntentStatus,
}

#[cw_serde]
pub enum IntentStatus {
    Activing,
    Listing,
    Transferring,
    Clearing,
    Cleared,
    Redeeming,
    Redeemed,
    Stopping,
}
