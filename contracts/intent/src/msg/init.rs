use cosmwasm_schema::cw_serde;
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {
    // pub staked_coin: Coin,
    pub leverage: u8,
    pub bollar_vault: String,
}

impl InstantiateMsg {
    pub fn new(leverage: u8, bollar_vault: String) -> Self {
        Self {
            // staked_coin,
            bollar_vault,
            leverage,
        }
    }
}
