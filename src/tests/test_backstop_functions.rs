#![cfg(test)]

use blend_contract_sdk::backstop;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    token::StellarAssetClient,
    vec, Address, Env, IntoVal, Symbol,
};

use crate::testutils::{create_blend_contracts, create_blend_lockup_wasm, EnvTestUtils};

#[test]
fn test_execute_backstop_functions() {
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
    let (_, blend_lockup_client) = create_blend_lockup_wasm(
        &e,
        &frodo,
        &contracts.emitter.address,
        &(e.ledger().timestamp() + 42 * 24 * 60 * 60),
        &Address::generate(&e),
    );

    // mint backstop tokens to the blend lockup contract
    let lp_mint_amount = 100_0000000;
    blnd_admin_client.mint(&blend_lockup_client.address, &100000_0000000);
    usdc_admin_client.mint(&blend_lockup_client.address, &2500_0000000);
    blend_lockup_client.c_join_pool(
        &contracts.backstop_token.address,
        &lp_mint_amount,
        &vec![&e, 100000_0000000, 2500_0000000],
    );

    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_deposit",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    contracts.backstop_token.address.clone().into_val(&e),
                    pool.clone().into_val(&e),
                    lp_mint_amount.clone().into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_deposit(
            &contracts.backstop.address,
            &contracts.backstop_token.address,
            &pool,
            &lp_mint_amount,
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "b_deposit"),
                    vec![
                        &e,
                        contracts.backstop.address.clone().into_val(&e),
                        contracts.backstop_token.address.clone().into_val(&e),
                        pool.clone().into_val(&e),
                        lp_mint_amount.clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );
    assert_eq!(
        lp_mint_amount,
        contracts
            .backstop
            .user_balance(&pool, &blend_lockup_client.address)
            .shares
    );

    e.jump(6000);
    contracts.emitter.distribute();
    contracts.backstop.gulp_emissions();

    e.jump(6000);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_claim",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    vec![&e, pool.clone()].into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_claim(&contracts.backstop.address, &vec![&e, pool.clone()]);
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "b_claim"),
                    vec![
                        &e,
                        contracts.backstop.address.clone().into_val(&e),
                        vec![&e, pool.clone()].into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );
    let new_shares = contracts
        .backstop
        .user_balance(&pool, &blend_lockup_client.address)
        .shares;
    assert!(new_shares > lp_mint_amount);
    let claim_amount = new_shares - lp_mint_amount;

    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_queue_withdrawal",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    pool.clone().into_val(&e),
                    (lp_mint_amount + claim_amount).clone().into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_queue_withdrawal(
            &contracts.backstop.address,
            &pool,
            &(lp_mint_amount + claim_amount),
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "b_queue_withdrawal"),
                    vec![
                        &e,
                        contracts.backstop.address.clone().into_val(&e),
                        pool.clone().into_val(&e),
                        (lp_mint_amount + claim_amount).clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    assert_eq!(
        Some(backstop::Q4W {
            amount: lp_mint_amount + claim_amount,
            exp: e.ledger().timestamp() + 21 * 24 * 60 * 60
        }),
        contracts
            .backstop
            .user_balance(&pool, &blend_lockup_client.address)
            .q4w
            .get(0)
    );

    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_dequeue_withdrawal",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    pool.clone().into_val(&e),
                    (lp_mint_amount + claim_amount).clone().into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_dequeue_withdrawal(
            &contracts.backstop.address,
            &pool,
            &(lp_mint_amount + claim_amount),
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "b_dequeue_withdrawal"),
                    vec![
                        &e,
                        contracts.backstop.address.clone().into_val(&e),
                        pool.clone().into_val(&e),
                        (lp_mint_amount + claim_amount).clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );
    assert_eq!(
        None,
        contracts
            .backstop
            .user_balance(&pool, &blend_lockup_client.address)
            .q4w
            .get(0)
    );

    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_queue_withdrawal",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    pool.clone().into_val(&e),
                    (lp_mint_amount + claim_amount).clone().into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_queue_withdrawal(
            &contracts.backstop.address,
            &pool,
            &(lp_mint_amount + claim_amount),
        );
    e.jump(21 * 24 * 60 * 60 / 5);

    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"b_withdraw",
                args: vec![
                    &e,
                    contracts.backstop.address.clone().into_val(&e),
                    pool.clone().into_val(&e),
                    (lp_mint_amount + claim_amount).clone().into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .b_withdraw(
            &contracts.backstop.address,
            &pool,
            &(lp_mint_amount + claim_amount),
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "b_withdraw"),
                    vec![
                        &e,
                        contracts.backstop.address.clone().into_val(&e),
                        pool.clone().into_val(&e),
                        (lp_mint_amount + claim_amount).clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    assert_eq!(
        0,
        contracts
            .backstop
            .user_balance(&pool, &blend_lockup_client.address)
            .shares
    );
    assert_eq!(
        lp_mint_amount + claim_amount,
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address)
    );
}
