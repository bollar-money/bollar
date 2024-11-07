use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum ExecuteMsg {
    // Stake BBN to create Intent contract
    Stake {},
}
