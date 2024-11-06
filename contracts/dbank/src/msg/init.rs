use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub bollar_vault: String,
    pub denoms: Vec<String>,
}

impl InstantiateMsg {
    pub fn new(bollar_vault: String, denoms: Vec<String>) -> Self {
        Self {
            bollar_vault,
            denoms,
        }
    }
}
