// #[cfg(test)]
// mod tests {

use crate::multitest::BollarCodeId;
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::{coin, coins, Uint128};

#[test]
fn bollar_should_works() {
    let ubbn_denom = "ubbn";

    let mut app = BabylonApp::new("bollar");

    let alice = app.api().addr_make("alice");

    app.init_modules(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &alice, coins(300 * 1_000_000_000, ubbn_denom))
            .unwrap();
    });

    let code_id = BollarCodeId::store_code(&mut app);
    let name = "Bollar";
    let symbol = "BOLLAR";
    let decimals = 9;
    let amount = Uint128::new(10_000_000_000 * 1_000_000_000);

    let bob = app.api().addr_make("bob");

    let label = "bollarvault";

    let contract = code_id
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

    let info_resp = contract.token_info(&app).unwrap();
    assert_eq!(info_resp.name, name);
    assert_eq!(info_resp.total_supply.u128(), amount.u128());
    assert_eq!(info_resp.decimals, 9);

    let circulating_resp = contract.query_circulating_shares(&app).unwrap();
    assert_eq!(circulating_resp.total_supply.u128(), amount.u128());
    assert_eq!(circulating_resp.circulating_shares.u128(), 0);

    // // mint
    // contract
    //     .mint(
    //         &mut app,
    //         alice.clone(),
    //         alice.clone().to_string(),
    //         amount,
    //         &[],
    //     )
    //     .unwrap();

    // query balance
    let balance_resp = contract
        .query_balance(&app, alice.clone().to_string())
        .unwrap();

    assert_eq!(balance_resp.u128(), 0);

    // exchange bollar
    let expected_bollar = 1000 * 1_000_000_000;
    let exchange_funds = coin(100 * 1_000_000_000, ubbn_denom);

    contract
        .set_exchange_rate(
            &mut app,
            alice.clone(),
            ubbn_denom.to_string(),
            Uint128::new(10),
            &[],
        )
        .unwrap();

    let rate_resp = contract
        .query_exchange_rate(&app, ubbn_denom.to_string())
        .unwrap();
    assert_eq!(rate_resp.u128(), 10);

    contract
        .exchange(&mut app, alice.clone(), &[exchange_funds])
        .unwrap();

    let circulating_resp = contract.query_circulating_shares(&app).unwrap();
    assert_eq!(circulating_resp.total_supply.u128(), amount.u128());
    assert_eq!(circulating_resp.circulating_shares.u128(), expected_bollar);

    let balance_resp = contract
        .query_balance(&app, alice.clone().to_string())
        .unwrap();

    assert_eq!(balance_resp.u128(), expected_bollar);

    let contract_balance_resp = contract
        .query_balance(&app, contract.addr().to_string())
        .unwrap();

    assert_eq!(
        contract_balance_resp.u128(),
        (10_000_000_000 - 1000) * 1_000_000_000
    );

    // transfer to bob
    contract
        .transfer(
            &mut app,
            alice.clone(),
            bob.to_string(),
            Uint128::new(3_000_000_000),
            &[],
        )
        .unwrap();

    let alice_resp = contract
        .query_balance(&app, alice.clone().to_string())
        .unwrap();
    let bob_resp = contract
        .query_balance(&app, bob.clone().to_string())
        .unwrap();

    assert_eq!(alice_resp.u128(), 997_000_000_000);
    assert_eq!(bob_resp.u128(), 3_000_000_000);

    // transfer to contract
    contract
        .transfer(
            &mut app,
            alice.clone(),
            contract.addr().to_string(),
            Uint128::new(1_000_000_000),
            &[],
        )
        .unwrap();

    let contract_resp = contract
        .query_balance(&app, contract.addr().to_string())
        .unwrap();

    assert_eq!(
        contract_resp.u128(),
        (10_000_000_000 - 1000 + 1) * 1_000_000_000
    );
}
