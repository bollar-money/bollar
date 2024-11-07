// #[cfg(test)]
// mod tests {

use crate::{models::IntentStatus, multitest::IntentCodeId};
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{coin, coins, Uint128};

#[test]
fn intent_should_works() {
    let ubbn_denom = "ubbn";

    let mut app = BabylonApp::new("dbank");

    let alice = app.api().addr_make("alice");

    app.init_modules(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice, coins(300 * 1_000_000_000, ubbn_denom))
            .unwrap();
    });

    let bollar_id = bollar::multitest::BollarCodeId::store_code(&mut app);

    let intent_id = IntentCodeId::store_code(&mut app);
    let name = "Bollar";
    let symbol = "BOLLAR";
    let decimals = 9;
    let amount = Uint128::zero();

    let staked_amount = 100 * 1_000_000_000;
    let staked_coin = coin(staked_amount, ubbn_denom);
    let leverage = 2;

    let intent_name = "bollar-intent";
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

    let intent_contract = intent_id
        .instantiate(
            &mut app,
            bollar_contract.addr().as_str(),
            leverage,
            intent_name,
            alice.clone(),
            staked_coin,
            label,
        )
        .unwrap();

    let metadata_resp = intent_contract.query_metadata(&app).unwrap().metadata;

    assert_eq!(metadata_resp.bollar_vault, bollar_contract.addr());
    assert_eq!(metadata_resp.creator, alice);
    assert_eq!(metadata_resp.status, IntentStatus::Activing);

    let balance_resp = intent_contract
        .query_balance(&app, intent_contract.addr().to_string(), "ubbn".to_string())
        .unwrap();
    assert_eq!(balance_resp.amount.u128(), staked_amount);
}

#[test]
fn intent_instantiate_should_works() {
    let ubbn_denom = "ubbn";

    let mut app = BabylonApp::new("dbank");

    let alice = app.api().addr_make("alice");

    let bollar_vault = app.api().addr_make("bollar");

    app.init_modules(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice, coins(300 * 1_000_000_000, ubbn_denom))
            .unwrap();
    });

    let staked_amount = 100 * 1_000_000_000;
    let staked_coin = coin(staked_amount, ubbn_denom);
    let leverage = 2;

    let intent_name = "bollar-intent";

    let label = "bollarvault";

    let intent_id = IntentCodeId::store_code(&mut app);
    let intent_contract = intent_id
        .instantiate(
            &mut app,
            bollar_vault.as_str(),
            leverage,
            intent_name,
            alice.clone(),
            staked_coin,
            label,
        )
        .unwrap();

    let metadata_resp = intent_contract.query_metadata(&app).unwrap().metadata;

    assert_eq!(metadata_resp.bollar_vault, bollar_vault);
    assert_eq!(metadata_resp.creator, alice);
    assert_eq!(metadata_resp.status, IntentStatus::Activing);

    // Check intent contract balance
    let contract_balance = intent_contract
        .query_balance(
            &app,
            intent_contract.addr().to_string(),
            ubbn_denom.to_string(),
        )
        .unwrap();
    assert_eq!(contract_balance.amount.u128(), staked_amount);
}
