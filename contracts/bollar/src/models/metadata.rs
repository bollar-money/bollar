use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct Metadata {
    pub token_denom: String,
    pub token_contract_address: Addr,
}

impl Metadata {
    pub fn new(token_denom: String, token_contract_address: Addr) -> Self {
        Self {
            token_denom,
            token_contract_address,
        }
    }
}
