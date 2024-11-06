use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    /// name of the token, should be `Bollar`
    pub name: String,
    /// symbol/ticker of the token
    pub symbol: String,

    pub decimals: u8,

    pub amount: Uint128,
}

impl InstantiateMsg {
    pub fn new(name: impl Into<String>, symbol: impl Into<String>, decimals: u8, amount: Uint128) -> Self {
        Self {
            name: name.into(),
            symbol: symbol.into(),
            decimals,
            amount,
        }
    }
}
