use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub leverage: u8,
    pub bollar_vault: String,
}

impl InstantiateMsg {
    pub fn new(name: String, leverage: u8, bollar_vault: String) -> Self {
        Self {
            name,
            bollar_vault,
            leverage,
        }
    }
}
