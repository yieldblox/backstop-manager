#![cfg(test)]

use blend_contract_sdk::testutils::BlendFixture;
use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::testutils::{create_blend_lockup_wasm, EnvTestUtils};

#[test]
fn test_execute_comet_functions() {
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
    let contracts = BlendFixture::deploy(&e, &bombadil, &blnd_id, &usdc_id);
    let (_, blend_lockup_client) = create_blend_lockup_wasm(
        &e,
        &frodo,
        &contracts.emitter.address,
        &(e.ledger().timestamp() + 42 * 24 * 60 * 60),
        &Address::generate(&e),
    );

    // mint underlying tokens to the blend lockup contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    let lp_mint_amount = 100_0000000;
    blnd_admin_client.mint(&blend_lockup_client.address, &100_000_0000000);
    usdc_admin_client.mint(&blend_lockup_client.address, &2500_0000000);
    let mut blnd_balance = 100_000_0000000;
    let mut usdc_balance = 2500_0000000;
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);
    let usdc_token = TokenClient::new(&e, &usdc_admin_client.address);

    // join pool
    let max_amount_in: Vec<i128> = vec![&e, 1010_0000000, 26_0000000];
    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"c_join_pool",
                args: vec![
                    &e,
                    contracts.backstop_token.address.clone().into_val(&e),
                    lp_mint_amount.into_val(&e),
                    max_amount_in.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_join_pool(
            &contracts.backstop_token.address,
            &lp_mint_amount,
            &max_amount_in,
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "c_join_pool"),
                    vec![
                        &e,
                        contracts.backstop_token.address.clone().into_val(&e),
                        lp_mint_amount.clone().into_val(&e),
                        max_amount_in.clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );
    let actual_blnd_in = blnd_balance - blnd_token.balance(&blend_lockup_client.address);
    let actual_usdc_in = usdc_balance - usdc_token.balance(&blend_lockup_client.address);
    assert!(
        blnd_token.balance(&blend_lockup_client.address)
            >= blnd_balance - max_amount_in.get_unchecked(0)
    );
    assert!(
        usdc_token.balance(&blend_lockup_client.address)
            >= usdc_balance - max_amount_in.get_unchecked(1)
    );
    assert_eq!(
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address),
        lp_mint_amount
    );
    blnd_balance = blnd_token.balance(&blend_lockup_client.address);
    usdc_balance = usdc_token.balance(&blend_lockup_client.address);

    // exit pool
    let min_amount_out: Vec<i128> =
        vec![&e, actual_blnd_in - 1_0000000, actual_usdc_in - 0_5000000];
    e.set_auths(&[]);
    blend_lockup_client
        .mock_auths(&[MockAuth {
            address: &frodo,
            invoke: &MockAuthInvoke {
                contract: &blend_lockup_client.address,
                fn_name: &"c_exit_pool",
                args: vec![
                    &e,
                    contracts.backstop_token.address.clone().into_val(&e),
                    lp_mint_amount.into_val(&e),
                    min_amount_out.into_val(&e),
                ],
                sub_invokes: &[],
            },
        }])
        .c_exit_pool(
            &contracts.backstop_token.address,
            &lp_mint_amount,
            &min_amount_out,
        );
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "c_exit_pool"),
                    vec![
                        &e,
                        contracts.backstop_token.address.clone().into_val(&e),
                        lp_mint_amount.clone().into_val(&e),
                        min_amount_out.clone().into_val(&e),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    assert!(
        blnd_token.balance(&blend_lockup_client.address)
            >= blnd_balance + min_amount_out.get_unchecked(0)
    );
    assert!(
        usdc_token.balance(&blend_lockup_client.address)
            >= usdc_balance + min_amount_out.get_unchecked(1)
    );
    assert_eq!(
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address),
        0
    );
}
