#![cfg(test)]

use crate::testutils::{
    create_backstop_bootstrapper, create_backstop_manager_wasm, create_blend_contracts,
    EnvTestUtils,
};
use soroban_sdk::{
    testutils::{Address as _, EnvTestConfig, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, Error, IntoVal,
};

#[test]
fn test_execute_bootstrapper_functions() {
    let e = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    e.cost_estimate().budget().reset_unlimited();
    e.set_default_info();
    // auths get reset and tested for each manager function
    e.mock_all_auths();

    let bombadil = Address::generate(&e);
    let frodo = Address::generate(&e);
    let samwise = Address::generate(&e);
    let usdc = e.register_stellar_asset_contract_v2(bombadil.clone());
    let blnd = e.register_stellar_asset_contract_v2(bombadil.clone());
    let usdc_admin_client = StellarAssetClient::new(&e, &usdc.address());
    let blnd_admin_client = StellarAssetClient::new(&e, &blnd.address());
    let (contracts, pool) = create_blend_contracts(&e, &bombadil, &blnd.address(), &usdc.address());
    let bootstrapper = create_backstop_bootstrapper(&e, &contracts);
    let blnd_index: u32 = 0;

    // start manager (samwise) at scope 1
    let (_, manager_client) = create_backstop_manager_wasm(
        &e,
        &frodo,
        &samwise,
        &1,
        &bootstrapper.address,
        &contracts.backstop_token.address,
        &vec![&e, contracts.backstop.address.clone()],
        &vec![&e, pool.clone()],
    );

    // mint underlying tokens to the backstop manager contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    //  -> 1000 shares available to mint
    let blnd_balance_0 = 100_000_0000000;
    let usdc_balance_0 = 2500_0000000;
    blnd_admin_client.mint(&manager_client.address, &blnd_balance_0);
    usdc_admin_client.mint(&frodo, &usdc_balance_0);
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);
    let duration: u32 = 17280 + 1;
    let pair_min_usdc: i128 = 10_000_0000;

    // create_bootstrap - validate requires scope 2 as manager
    let blnd_bootstrap_amount = blnd_balance_0;
    e.set_auths(&[]);
    let bootstrap_scope_1 = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address.clone(),
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    blnd_index.into_val(&e),
                    blnd_bootstrap_amount.into_val(&e),
                    pair_min_usdc.into_val(&e),
                    duration.into_val(&e),
                    pool.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .try_bb_start_bootstrap(
            &samwise,
            &blnd_index,
            &blnd_bootstrap_amount,
            &pair_min_usdc,
            &duration,
            &pool,
        );
    assert_eq!(
        bootstrap_scope_1.err(),
        Some(Ok(Error::from_contract_error(4)))
    );

    /***** SCOPE 2 *****/

    // set manager (samwise) to scope 2
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"set_manager",
                args: vec![&e, samwise.into_val(&e), 2u32.into_val(&e)],
                sub_invokes: &[],
            },
        }])
        .set_manager(&samwise, &2u32);

    // assert scope is 2
    let manager = manager_client.manager();
    assert_eq!(2, manager.scope);
    assert_eq!(samwise, manager.id);

    // create_bootstrap - as manager
    e.set_auths(&[]);
    let id = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address.clone(),
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    blnd_index.into_val(&e),
                    blnd_bootstrap_amount.into_val(&e),
                    pair_min_usdc.into_val(&e),
                    duration.into_val(&e),
                    pool.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .bb_start_bootstrap(
            &samwise,
            &blnd_index,
            &blnd_bootstrap_amount,
            &pair_min_usdc,
            &duration,
            &pool,
        );
    assert_eq!(e.auths()[0].0, samwise); // assert require_auth exists
    assert_eq!(id, 0);
    let blnd_balance_1 = blnd_token.balance(&manager_client.address);
    assert_eq!(blnd_balance_1, blnd_balance_0 - blnd_bootstrap_amount);
    assert_eq!(
        blnd_token.balance(&bootstrapper.address),
        blnd_bootstrap_amount
    );
    let bootstrap = bootstrapper.get_bootstrap(&0);
    assert_eq!(bootstrap.data.bootstrap_amount, blnd_bootstrap_amount);

    // frodo join bootstrap
    e.mock_all_auths();
    bootstrapper.join(&frodo, &0, &usdc_balance_0);

    // claim bootstrap
    e.jump(duration + 1);
    bootstrapper.close(&0);

    e.set_auths(&[]);
    let bootstrap = bootstrapper.get_bootstrap(&0);
    let claim_amount: i128 = 79999992319;
    assert_eq!(
        claim_amount,
        bootstrap.data.total_backstop_tokens * 800_0000 as i128 / 1_000_0000
    );

    /***** SCOPE 0 *****/

    // set manager (samwise) to scope 0
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"set_manager",
                args: vec![&e, samwise.into_val(&e), 0u32.into_val(&e)],
                sub_invokes: &[],
            },
        }])
        .set_manager(&samwise, &0u32);

    // assert scope is 0
    let manager = manager_client.manager();
    assert_eq!(0, manager.scope);
    assert_eq!(samwise, manager.id);

    // claim bootstrap - as manager
    e.set_auths(&[]);
    let actual_claim_amount = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"bb_claim_bootstrap",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    bootstrap.id.into_val(&e),
                    contracts.backstop.address.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .bb_claim_bootstrap(&samwise, &bootstrap.id, &contracts.backstop.address);
    assert_eq!(e.auths()[0].0, samwise); // assert require_auth exists
    let backstop_balance_1 = contracts
        .backstop
        .user_balance(&pool, &manager_client.address);
    // shares are 1-1 with backstop tokens
    assert_eq!(actual_claim_amount, claim_amount);
    assert_eq!(backstop_balance_1.shares, claim_amount);
}

#[test]
fn test_execute_bootstrapper_functions_cancelled() {
    let e = Env::new_with_config(EnvTestConfig {
        capture_snapshot_at_drop: false,
    });
    e.cost_estimate().budget().reset_unlimited();
    e.set_default_info();
    // auths get reset and tested for each manager function
    e.mock_all_auths();

    let bombadil = Address::generate(&e);
    let frodo = Address::generate(&e);
    let samwise = Address::generate(&e);
    let usdc = e.register_stellar_asset_contract_v2(bombadil.clone());
    let blnd = e.register_stellar_asset_contract_v2(bombadil.clone());
    let usdc_admin_client = StellarAssetClient::new(&e, &usdc.address());
    let blnd_admin_client = StellarAssetClient::new(&e, &blnd.address());
    let (contracts, pool) = create_blend_contracts(&e, &bombadil, &blnd.address(), &usdc.address());
    let bootstrapper = create_backstop_bootstrapper(&e, &contracts);
    let blnd_index: u32 = 0;

    // start manager (samwise) at scope 0
    let (_, manager_client) = create_backstop_manager_wasm(
        &e,
        &frodo,
        &samwise,
        &0,
        &bootstrapper.address,
        &contracts.backstop_token.address,
        &vec![&e, contracts.backstop.address.clone()],
        &vec![&e, pool.clone()],
    );

    // mint underlying tokens to the backstop manager contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    //  -> 1000 shares available to mint
    let blnd_balance_0 = 100_000_0000000;
    let usdc_balance_0 = 2500_0000000;
    blnd_admin_client.mint(&manager_client.address, &blnd_balance_0);
    usdc_admin_client.mint(&manager_client.address, &usdc_balance_0);
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);
    let duration: u32 = 17280 + 1;
    let pair_min_usdc: i128 = 10_000_0000;

    // create_bootstrap - as owner
    let blnd_bootstrap_amount = blnd_balance_0;
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address.clone(),
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    frodo.into_val(&e),
                    blnd_index.into_val(&e),
                    blnd_bootstrap_amount.into_val(&e),
                    pair_min_usdc.into_val(&e),
                    duration.into_val(&e),
                    pool.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .bb_start_bootstrap(
            &frodo,
            &blnd_index,
            &blnd_bootstrap_amount,
            &pair_min_usdc,
            &duration,
            &pool,
        );
    assert_eq!(e.auths()[0].0, frodo); // assert require_auth exists
    let blnd_balance_1 = blnd_token.balance(&manager_client.address);
    assert_eq!(blnd_balance_1, blnd_balance_0 - blnd_bootstrap_amount);
    assert_eq!(
        blnd_token.balance(&bootstrapper.address),
        blnd_bootstrap_amount
    );
    let bootstrap = bootstrapper.get_bootstrap(&0);
    assert_eq!(bootstrap.data.bootstrap_amount, blnd_bootstrap_amount);

    // wait until bootstrap expires
    e.jump(duration + 1);

    // assert scope is 0
    let manager = manager_client.manager();
    assert_eq!(0, manager.scope);
    assert_eq!(samwise, manager.id);

    // refund bootstrap - as manager
    e.set_auths(&[]);
    let bootstrap_id = 0;
    let refund_amount = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"bb_refund_bootstrap",
                args: vec![&e, samwise.into_val(&e), bootstrap_id.into_val(&e)],
                sub_invokes: &[],
            },
        }])
        .bb_refund_bootstrap(&samwise, &bootstrap_id);
    assert_eq!(e.auths()[0].0, samwise); // assert require_auth exists
    assert_eq!(refund_amount, blnd_bootstrap_amount);
    assert_eq!(blnd_token.balance(&manager_client.address), blnd_balance_0)
}
