use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};

#[cw_serde]
pub struct Metadata {
    pub staked_token_denom: String,
    pub staked_token_amount: Uint128,
    pub lever: u8,
    pub creator: Addr,
    pub created_at: Timestamp,
    pub status: DBankStatus,
}

#[cw_serde]
pub enum DBankStatus {
    Activing,
    Stopped,
}

// impl Metadata {
//     pub fn new(token_denom: String, token_contract_address: Addr) -> Self {
//         Self {
//             token_denom,
//             token_contract_address,
//         }
//     }
// }
