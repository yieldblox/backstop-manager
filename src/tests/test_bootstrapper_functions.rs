#![cfg(test)]

use std::println;

use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, IntoVal,
};

use crate::{
    dependencies::bootstrapper::{BootstrapData, BootstrapKey},
    testutils::{create_blend_contracts, create_blend_lockup_wasm, EnvTestUtils},
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
    let contracts = create_blend_contracts(&e, &bombadil, &usdc_id, &blnd_id);
    let (_, blend_lockup_client) = create_blend_lockup_wasm(
        &e,
        &frodo,
        &contracts.emitter.address,
        &(e.ledger().timestamp() + 42 * 24 * 60 * 60),
        &contracts.bootstrapper.address,
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
    let usdc_balance = 2500_0000000;
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
                contract: &blend_lockup_client.address,
                fn_name: &"bb_start_bootstrap",
                args: vec![
                    &e,
                    blnd_index.clone().into_val(&e),
                    blnd_amount.clone().into_val(&e),
                    min.clone().into_val(&e),
                    duration.clone().into_val(&e),
                    contracts.pool.address.clone().into_val(&e),
                ],
                sub_invokes: &[MockAuthInvoke {
                    contract: &contracts.bootstrapper.address,
                    fn_name: &"add_bootstrap",
                    args: vec![
                        &e,
                        blend_lockup_client.address.clone().into_val(&e),
                        (0).into_val(&e),
                        blnd_amount.clone().into_val(&e),
                        (10_000_0000).into_val(&e),
                        duration.into_val(&e),
                        contracts.pool.address.into_val(&e),
                    ],
                    sub_invokes: &[MockAuthInvoke {
                        contract: &blnd_token.address,
                        fn_name: &"transfer",
                        args: vec![
                            &e,
                            blend_lockup_client.address.clone().into_val(&e),
                            contracts.bootstrapper.address.clone().into_val(&e),
                            blnd_amount.into_val(&e),
                        ],
                        sub_invokes: &[],
                    }],
                }],
            },
        }])
        .bb_start_bootstrap(
            &blnd_index,
            &blnd_amount,
            &min,
            &duration,
            &contracts.pool.address,
        );
    e.as_contract(&contracts.bootstrapper.address, || {
        let key = BootstrapKey {
            id: 0,
            creator: blend_lockup_client.address.clone(),
        };

        assert!(e
            .storage()
            .persistent()
            .get::<BootstrapKey, BootstrapData>(&key)
            .is_some());
    });
    assert_eq!(
        blnd_token.balance(&contracts.bootstrapper.address).clone(),
        blnd_amount.clone()
    );
    let backstop_token_balance = contracts
        .backstop_token
        .balance(&contracts.backstop.address);
    // frodo join bootstrap
    e.mock_all_auths();
    contracts
        .bootstrapper
        .join(&frodo, &usdc_balance, &blend_lockup_client.address, &0);

    // claim bootstrap
    e.jump(duration + 1);
    contracts
        .bootstrapper
        .close_bootstrap(&frodo, &blend_lockup_client.address, &0);

    e.set_auths(&[]);
    let bootstrap = contracts
        .bootstrapper
        .get_bootstrap(&0, &blend_lockup_client.address.clone());
    let claim_amount: i128 = 79999858378;
    assert_eq!(
        claim_amount,
        bootstrap.backstop_tokens * bootstrap.bootstrap_weight as i128 / 1_000_0000
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
                    contract: &contracts.bootstrapper.address,
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
                                contracts.bootstrapper.address.clone().into_val(&e),
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
                                contracts.pool.address.clone().into_val(&e),
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
