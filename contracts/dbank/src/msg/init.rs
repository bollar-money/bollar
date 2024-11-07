use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub bollar_vault: String,
    pub denoms: Vec<String>,
    pub intent_code_id: u64,
}

impl InstantiateMsg {
    pub fn new(bollar_vault: String, denoms: Vec<String>, intent_code_id: u64) -> Self {
        Self {
            bollar_vault,
            denoms,
            intent_code_id,
        }
    }
}
