use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {
    pub bollar_vault: String,
}

impl InstantiateMsg {
    pub fn new(bollar_vault: String) -> Self {
        Self {
            bollar_vault
        }
    }
}
