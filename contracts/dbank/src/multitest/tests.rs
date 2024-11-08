// #[cfg(test)]
// mod tests {

use crate::{models::DBankStatus, multitest::DBankCodeId};
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{coin, coins, Uint128};

#[test]
fn dbank_should_works() {
    let ubbn_denom = "ubbn";

    let mut app = BabylonApp::new("dbank");

    let alice = app.api().addr_make("alice");

    // let bob = app.api().addr_make("bob");

    app.init_modules(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice, coins(300 * 1_000_000_000, ubbn_denom))
            .unwrap();
    });

    let bollar_id = bollar::multitest::BollarCodeId::store_code(&mut app);
    let intent_id = intent::multitest::IntentCodeId::store_code(&mut app);

    let dbank_id = DBankCodeId::store_code(&mut app);
    let name = "Bollar";
    let symbol = "BOLLAR";
    let decimals = 9;
    let amount = Uint128::zero();

    let denoms = vec!["ubbn".to_string()];

    let label = "bollarvault";

    let bollar_contract = bollar_id
        .instantiate(
            &mut app,
            name,
            symbol,
            decimals,
            amount,
            alice.clone(),
            label,
        )
        .unwrap();

    let dbank_contract = dbank_id
        .instantiate(
            &mut app,
            bollar_contract.addr().to_string().as_str(),
            denoms,
            intent_id.into(),
            alice.clone(),
            label,
        )
        .unwrap();

    let metadata_resp = dbank_contract.query_metadata(&app).unwrap().metadata;

    assert_eq!(metadata_resp.bollar_vault, bollar_contract.addr());
    assert_eq!(metadata_resp.creator, alice);
    assert_eq!(metadata_resp.status, DBankStatus::Activing);

    let denoms_resp = dbank_contract.query_denoms(&app).unwrap();

    assert_eq!(denoms_resp.len(), 1);

    let balance_resp = dbank_contract
        .query_balance(&app, dbank_contract.addr().to_string(), "ubbn".to_string())
        .unwrap();
    assert_eq!(balance_resp.amount.u128(), 0);

    // Stake ubbn
    let coin = coin(2_000_000_000, "ubbn");
    let intent_name = "intent_alice";
    let leverage = 2;
    let intent_contract = dbank_contract
        .stake(
            &mut app,
            alice.clone(),
            &[coin.clone()],
            leverage,
            intent_name.to_string(),
        )
        .unwrap()
        .unwrap()
        .addr;

    println!("Created intent contract: {intent_contract:?}");

    let intent_info = dbank_contract.query_intent(&app, alice.to_string(), intent_contract.to_string()).unwrap();
    assert_eq!(intent_info.unwrap().leverage, 2);

    let contract_balance_resp = dbank_contract
        .query_balance(&app, dbank_contract.addr().to_string(), "ubbn".to_string())
        .unwrap();

    assert_eq!(contract_balance_resp.amount.u128(), 0);

    let alice_balance_resp = dbank_contract
        .query_balance(&app, alice.to_string(), "ubbn".to_string())
        .unwrap();

    assert!(alice_balance_resp.amount.u128() == 298 * 1_000_000_000);

    // check intent info
    let info_resp = dbank_contract
        .query_intents(&app, alice.to_string())
        .unwrap();
    assert_eq!(info_resp.len(), 1);
}
