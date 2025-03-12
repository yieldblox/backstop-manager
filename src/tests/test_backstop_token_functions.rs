#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, EnvTestConfig, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, Error, IntoVal, Vec,
};

use crate::testutils::{
    create_backstop_bootstrapper, create_backstop_manager_wasm, create_blend_contracts,
    EnvTestUtils,
};

#[test]
fn test_execute_comet_functions() {
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

    // start manager (samwise) at scope 1
    let (_, manager_client) = create_backstop_manager_wasm(
        &e,
        &frodo,
        &samwise,
        &1,
        &contracts.emitter.address,
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
    let usdc_token = TokenClient::new(&e, &usdc_admin_client.address);

    /***** SCOPE 1 *****/

    // join pool - as owner
    let lp_mint_amount = 100_0000000;
    let max_amount_in: Vec<i128> = vec![&e, 1001_0000000, 25_1000000];
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_join_pool",
                args: vec![
                    &e,
                    frodo.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    max_amount_in.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_join_pool(&frodo, &lp_mint_amount, &max_amount_in);
    assert_eq!(e.auths()[0].0, frodo); // assert require_auth exists
    let blnd_balance_1 = blnd_token.balance(&manager_client.address);
    let usdc_balance_1 = usdc_token.balance(&manager_client.address);
    let lp_balance_1 = contracts.backstop_token.balance(&manager_client.address);
    assert!(blnd_balance_1 >= blnd_balance_0 - max_amount_in.get_unchecked(0));
    assert!(usdc_balance_1 >= usdc_balance_0 - max_amount_in.get_unchecked(1));
    assert_eq!(lp_balance_1, lp_mint_amount);

    // join pool - validate requires scope 2 as manager
    e.set_auths(&[]);
    let join_scope_1 = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_join_pool",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    max_amount_in.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .try_c_join_pool(&samwise, &lp_mint_amount, &max_amount_in);
    assert_eq!(join_scope_1.err(), Some(Ok(Error::from_contract_error(4))));

    // exit pool - validate requires scope 2 as manager
    let min_amount_out: Vec<i128> = vec![&e, 999_0000000, 24_9000000];
    let exit_scope_1 = manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_exit_pool",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    min_amount_out.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .try_c_exit_pool(&samwise, &lp_mint_amount, &min_amount_out);
    assert_eq!(exit_scope_1.err(), Some(Ok(Error::from_contract_error(4))));

    // exit pool - as owner
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_exit_pool",
                args: vec![
                    &e,
                    frodo.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    min_amount_out.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_exit_pool(&frodo, &lp_mint_amount, &min_amount_out);
    assert_eq!(e.auths()[0].0, frodo); // assert require_auth exists
    let blnd_balance_2 = blnd_token.balance(&manager_client.address);
    let usdc_balance_2 = usdc_token.balance(&manager_client.address);
    let lp_balance_2 = contracts.backstop_token.balance(&manager_client.address);
    assert!(blnd_balance_2 >= blnd_balance_1 + min_amount_out.get_unchecked(0));
    assert!(usdc_balance_2 >= usdc_balance_1 + min_amount_out.get_unchecked(1));
    assert_eq!(lp_balance_2, 0);

    /***** SCOPE 2 *****/

    // set manager (samwise) at scope 2
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

    let manager = manager_client.manager();
    assert_eq!(manager.id, samwise);
    assert_eq!(manager.scope, 2);

    // join pool - as manager
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_join_pool",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    max_amount_in.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_join_pool(&samwise, &lp_mint_amount, &max_amount_in);
    assert_eq!(e.auths()[0].0, samwise); // assert require_auth exists
    let blnd_balance_3 = blnd_token.balance(&manager_client.address);
    let usdc_balance_3 = usdc_token.balance(&manager_client.address);
    let lp_balance_3 = contracts.backstop_token.balance(&manager_client.address);
    assert!(blnd_balance_3 >= blnd_balance_2 - max_amount_in.get_unchecked(0));
    assert!(usdc_balance_3 >= usdc_balance_2 - max_amount_in.get_unchecked(1));
    assert_eq!(lp_balance_3, lp_mint_amount);

    // exit pool - as manager
    e.set_auths(&[]);
    manager_client
        .mock_auths(&[MockAuth {
            address: &samwise,
            invoke: &MockAuthInvoke {
                contract: &manager_client.address,
                fn_name: &"c_exit_pool",
                args: vec![
                    &e,
                    samwise.into_val(&e),
                    lp_mint_amount.into_val(&e),
                    min_amount_out.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_exit_pool(&samwise, &lp_mint_amount, &min_amount_out);
    assert_eq!(e.auths()[0].0, samwise); // assert require_auth exists
    let blnd_balance_4 = blnd_token.balance(&manager_client.address);
    let usdc_balance_4 = usdc_token.balance(&manager_client.address);
    let lp_balance_4 = contracts.backstop_token.balance(&manager_client.address);
    assert!(blnd_balance_4 >= blnd_balance_3 + min_amount_out.get_unchecked(0));
    assert!(usdc_balance_4 >= usdc_balance_3 + min_amount_out.get_unchecked(1));
    assert_eq!(lp_balance_4, 0);
}
