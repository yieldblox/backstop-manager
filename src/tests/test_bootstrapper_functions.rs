#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, IntoVal,
};
use std::println;

use crate::{
    dependencies::bootstrapper::BootstrapConfig,
    testutils::{
        create_backstop_bootstrapper, create_blend_contracts, create_blend_lockup_wasm,
        EnvTestUtils,
    },
};

#[test]
fn test_execute_bootstrapper_functions() {
    let e = Env::default();
    e.budget().reset_unlimited();
    e.set_default_info();
    e.mock_all_auths();

    let bombadil = Address::generate(&e);
    let frodo = Address::generate(&e);
    let usdc_id = e.register_stellar_asset_contract(bombadil.clone());
    let blnd_id = e.register_stellar_asset_contract(bombadil.clone());
    let usdc_admin_client = StellarAssetClient::new(&e, &usdc_id);
    let blnd_admin_client = StellarAssetClient::new(&e, &blnd_id);
    let (contracts, pool) = create_blend_contracts(&e, &bombadil, &blnd_id, &usdc_id);
    let bootstrapper = create_backstop_bootstrapper(&e, &contracts);
    let (_, blend_lockup_client) = create_blend_lockup_wasm(
        &e,
        &frodo,
        &contracts.emitter.address,
        &(e.ledger().timestamp() + 42 * 24 * 60 * 60),
        &bootstrapper.address,
    );

    // mint underlying tokens to the blend lockup contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    let blnd_amount = &100_000_0000000;
    let usdc_amount = &2500_0000000;
    let blnd_index: u32 = 0;
    blnd_admin_client.mint(&blend_lockup_client.address, blnd_amount);
    usdc_admin_client.mint(&frodo, usdc_amount);
    let usdc_balance: i128 = 2500_0000000;
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);
    let duration: u32 = 17280 + 1;
    let min: i128 = 10_000_0000;
    // create_bootstrap
    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address.clone(),
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    blnd_index.clone().into_val(&e),
                    blnd_amount.clone().into_val(&e),
                    min.clone().into_val(&e),
                    duration.clone().into_val(&e),
                    pool.clone().into_val(&e),
                ],
                sub_invokes: &[MockAuthInvoke {
                    contract: &bootstrapper.address,
                    fn_name: &"bootstrap",
                    args: vec![
                        &e,
                        BootstrapConfig {
                            token_index: 0,
                            bootstrapper: blend_lockup_client.address.clone(),
                            amount: blnd_amount.clone(),
                            pair_min: 10_000_0000,
                            close_ledger: (e.ledger().timestamp() + 17280 + 1) as u32,
                            pool: pool.clone(),
                        }
                        .into_val(&e),
                    ],
                    sub_invokes: &[MockAuthInvoke {
                        contract: &blnd_token.address,
                        fn_name: &"transfer",
                        args: vec![
                            &e,
                            blend_lockup_client.address.clone().into_val(&e),
                            bootstrapper.address.clone().into_val(&e),
                            blnd_amount.into_val(&e),
                        ],
                        sub_invokes: &[],
                    }],
                }],
            },
        }])
        .bb_start_bootstrap(&blnd_index, &blnd_amount, &min, &duration, &pool);

    assert_eq!(
        blnd_token.balance(&bootstrapper.address).clone(),
        blnd_amount.clone()
    );
    let backstop_token_balance = contracts
        .backstop_token
        .balance(&contracts.backstop.address);
    // frodo join bootstrap
    e.mock_all_auths();
    bootstrapper.join(&frodo, &0, &usdc_balance);

    // claim bootstrap
    e.jump(duration + 1);
    bootstrapper.close(&0);

    e.set_auths(&[]);
    let bootstrap = bootstrapper.get_bootstrap(&0);
    let claim_amount: i128 = 79999567983;
    assert_eq!(
        claim_amount,
        bootstrap.data.total_backstop_tokens * 800_0000 as i128 / 1_000_0000
    );
    let bootstrap_id = 0;

    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"bb_claim_bootstrap",
                args: vec![&e, bootstrap_id.clone().into_val(&e)],
                sub_invokes: &[MockAuthInvoke {
                    contract: &bootstrapper.address,
                    fn_name: &"claim",
                    args: vec![
                        &e,
                        blend_lockup_client.address.clone().into_val(&e),
                        blend_lockup_client.address.clone().into_val(&e),
                        bootstrap_id.clone().into_val(&e),
                    ],
                    sub_invokes: &[
                        MockAuthInvoke {
                            contract: &contracts.backstop_token.address,
                            fn_name: &"transfer",
                            args: vec![
                                &e,
                                bootstrapper.address.clone().into_val(&e),
                                blend_lockup_client.address.clone().into_val(&e),
                                claim_amount.clone().into_val(&e),
                            ],
                            sub_invokes: &[],
                        },
                        MockAuthInvoke {
                            contract: &contracts.backstop.address,
                            fn_name: &"deposit",
                            args: vec![
                                &e,
                                blend_lockup_client.address.clone().into_val(&e),
                                pool.clone().into_val(&e),
                                claim_amount.clone().into_val(&e),
                            ],
                            sub_invokes: &[MockAuthInvoke {
                                contract: &contracts.backstop_token.address,
                                fn_name: &"transfer",
                                args: vec![
                                    &e,
                                    blend_lockup_client.address.clone().into_val(&e),
                                    contracts.backstop.address.clone().into_val(&e),
                                    claim_amount.clone().into_val(&e),
                                ],
                                sub_invokes: &[],
                            }],
                        },
                    ],
                }],
            },
        }])
        .bb_claim_bootstrap(&bootstrap_id);

    assert_eq!(
        contracts
            .backstop_token
            .balance(&contracts.backstop.address),
        backstop_token_balance + claim_amount
    )
}

#[test]
fn test_execute_bootstrapper_functions_cancelled() {
    let e = Env::default();
    e.budget().reset_unlimited();
    e.set_default_info();
    e.mock_all_auths();

    let bombadil = Address::generate(&e);
    let frodo = Address::generate(&e);
    let usdc_id = e.register_stellar_asset_contract(bombadil.clone());
    let blnd_id = e.register_stellar_asset_contract(bombadil.clone());
    let usdc_admin_client = StellarAssetClient::new(&e, &usdc_id);
    let blnd_admin_client = StellarAssetClient::new(&e, &blnd_id);
    let (contracts, pool) = create_blend_contracts(&e, &bombadil, &blnd_id, &usdc_id);
    let bootstrapper = create_backstop_bootstrapper(&e, &contracts);
    let (_, blend_lockup_client) = create_blend_lockup_wasm(
        &e,
        &frodo,
        &contracts.emitter.address,
        &(e.ledger().timestamp() + 42 * 24 * 60 * 60),
        &bootstrapper.address,
    );

    // mint underlying tokens to the blend lockup contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    let blnd_amount = &100_000_0000000;
    let usdc_amount = &2500_0000000;
    let blnd_index: u32 = 0;
    blnd_admin_client.mint(&blend_lockup_client.address, blnd_amount);
    usdc_admin_client.mint(&frodo, usdc_amount);
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);
    let duration: u32 = 17280 + 1;
    let min: i128 = 10_000_0000;
    // create_bootstrap
    println!("creating bootstrap");
    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address.clone(),
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    blnd_index.clone().into_val(&e),
                    blnd_amount.clone().into_val(&e),
                    min.clone().into_val(&e),
                    duration.clone().into_val(&e),
                    pool.clone().into_val(&e),
                ],
                sub_invokes: &[MockAuthInvoke {
                    contract: &bootstrapper.address,
                    fn_name: &"bootstrap",
                    args: vec![
                        &e,
                        BootstrapConfig {
                            token_index: 0,
                            bootstrapper: blend_lockup_client.address.clone(),
                            amount: blnd_amount.clone(),
                            pair_min: 10_000_0000,
                            close_ledger: (e.ledger().timestamp() + 17280 + 1) as u32,
                            pool: pool.clone(),
                        }
                        .into_val(&e),
                    ],
                    sub_invokes: &[MockAuthInvoke {
                        contract: &blnd_token.address,
                        fn_name: &"transfer",
                        args: vec![
                            &e,
                            blend_lockup_client.address.clone().into_val(&e),
                            bootstrapper.address.clone().into_val(&e),
                            blnd_amount.into_val(&e),
                        ],
                        sub_invokes: &[],
                    }],
                }],
            },
        }])
        .bb_start_bootstrap(&blnd_index, &blnd_amount, &min, &duration, &pool);

    assert_eq!(
        blnd_token.balance(&bootstrapper.address).clone(),
        blnd_amount.clone()
    );

    // wait until bootstrap expires
    e.jump(duration + 1);
    e.set_auths(&[]);
    let bootstrap_id = 0;
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"bb_refund_bootstrap",
                args: vec![&e, bootstrap_id.clone().into_val(&e)],
                sub_invokes: &[MockAuthInvoke {
                    contract: &bootstrapper.address,
                    fn_name: &"refund",
                    args: vec![
                        &e,
                        blend_lockup_client.address.clone().into_val(&e),
                        bootstrap_id.clone().into_val(&e),
                    ],
                    sub_invokes: &[MockAuthInvoke {
                        contract: &contracts.backstop_token.address,
                        fn_name: &"transfer",
                        args: vec![
                            &e,
                            bootstrapper.address.clone().into_val(&e),
                            blend_lockup_client.address.clone().into_val(&e),
                            blnd_amount.clone().into_val(&e),
                        ],
                        sub_invokes: &[],
                    }],
                }],
            },
        }])
        .bb_refund_bootstrap(&bootstrap_id);

    assert_eq!(
        blnd_token.balance(&blend_lockup_client.address).clone(),
        blnd_amount.clone()
    )
}
