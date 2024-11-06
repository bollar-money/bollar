// #[cfg(test)]
// mod tests {

use crate::{models::DBankStatus, multitest::DBankCodeId};
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::Uint128;

#[test]
fn dbank_should_works() {
    
    let mut app = BabylonApp::new("dbank");

    let bollar_id = bollar::multitest::BollarCodeId::store_code(&mut app);
    
    let dbank_id = DBankCodeId::store_code(&mut app);
    let name = "Bollar";
    let symbol = "BOLLAR";
    let decimals = 9;
    let amount = Uint128::zero();

    let alice = app
        .api()
        .addr_make("alice");
    
    let bob = app
        .api()
        .addr_make("bob");

    let label = "bollarvault";

    let bollar_contract = bollar_id
        .instantiate(&mut app, name, symbol, decimals, amount,alice.clone(), label)
        .unwrap();

    let dbank_contract = dbank_id
        .instantiate(&mut app, bollar_contract.addr().to_string().as_str(), alice.clone(), label).unwrap();

    let metadata_resp = dbank_contract.metadata(&app).unwrap().metadata;

    assert_eq!(metadata_resp.bollar_vault, bollar_contract.addr());
    assert_eq!(metadata_resp.creator, alice);
    assert_eq!(metadata_resp.status, DBankStatus::Activing);

}

