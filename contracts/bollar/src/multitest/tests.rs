// #[cfg(test)]
// mod tests {

use crate::multitest::BollarCodeId;
use babylon_bindings_test::BabylonApp;
use cosmwasm_std::Uint128;

#[test]
fn bollar_should_works() {
    // let mut app = App::default();
    let mut app = BabylonApp::new("alice");

    let code_id = BollarCodeId::store_code(&mut app);
    let name = "Bollar";
    let symbol = "BOLLAR";
    let decimals = 9;
    let amount = Uint128::zero();

    let alice = app
        .api()
        .addr_make("bbn1mc7wvxw0xnze3nngg05uav50fx6tew6glfplvx");
    let bob = app
        .api()
        .addr_make("bbn1egk0rj0vgeht3mnly8xzy0hu5a79dmsgc74mqt");
    let label = "bollarvault";

    let contract = code_id
        .instantiate(&mut app, name, symbol, decimals, amount,alice.clone(), label)
        .unwrap();

    let info_resp = contract.token_info(&app).unwrap();
    assert_eq!(info_resp.name, name);
    assert_eq!(info_resp.total_supply.u128(), 0);
    assert_eq!(info_resp.decimals, 9);

    let amount = Uint128::new(10_000_000_000);

    // mint
    contract
        .mint(
            &mut app,
            alice.clone(),
            alice.clone().to_string(),
            amount,
            &[],
        )
        .unwrap();

    // query balance
    let balance_resp = contract
        .query_balance(&app, alice.clone().to_string())
        .unwrap();

    assert_eq!(balance_resp.u128(), amount.u128());

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

    assert_eq!(alice_resp.u128(), 7_000_000_000);
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

    assert_eq!(contract_resp.u128(), 1_000_000_000);

}

