use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp};

#[cw_serde]
pub struct Metadata {
    pub bollar_vault: Addr,
    pub intent_code_id: u64,
    pub creator: Addr,
    pub created_at: Timestamp,
    pub status: DBankStatus,
}

#[cw_serde]
pub enum DBankStatus {
    Activing,
    Stopped,
}
