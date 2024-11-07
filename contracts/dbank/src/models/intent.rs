use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct IntentInfo {
    pub name: String,
    pub bollar_vault: Addr,
    pub leverage: u8,
    pub creator: Addr,
    pub created_at: Timestamp,
}

#[cw_serde]
pub struct IntentInstantiateMsg {
    pub name: String,
    pub leverage: u8,
    pub bollar_vault: String,
}
