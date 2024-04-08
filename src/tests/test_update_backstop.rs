#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    token::StellarAssetClient,
    vec, Address, Env, Error, Symbol,
};

use crate::testutils::{
    create_blend_contracts, create_blend_lockup_wasm, EnvTestUtils, ONE_DAY_LEDGERS,
};

#[test]
fn test_update_backstop() {
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

    let starting_blnd_balance: i128 = 100_000_0000000;
    let starting_usdc_balance: i128 = 2500_0000000;
    blnd_admin_client.mint(&blend_lockup_client.address, &starting_blnd_balance);
    usdc_admin_client.mint(&blend_lockup_client.address, &starting_usdc_balance);

    // mint underlying tokens to the blend lockup contract
    // create_blend_contracts sets up a comet LP with
    //  -> 10 BLND / share
    //  -> 0.25 USDC / share
    let lp_mint_amount = 1_000_0000000;
    blend_lockup_client.c_join_pool(
        &contracts.backstop_token.address,
        &lp_mint_amount,
        &vec![&e, 11_000_0000000, 260_0000000],
    );

    blend_lockup_client.b_deposit(
        &contracts.backstop.address,
        &contracts.backstop_token.address,
        &contracts.pool.address,
        &lp_mint_amount,
    );
    assert_eq!(
        lp_mint_amount,
        contracts
            .backstop
            .user_balance(&contracts.pool.address, &blend_lockup_client.address)
            .shares
    );

    // mint a bunch of tokens to bombadil and start a backstop swap
    blnd_admin_client.mint(&bombadil, &1_500_000_0000000);
    usdc_admin_client.mint(&bombadil, &40000_0000000);
    contracts.backstop_token.join_pool(
        &60_000_0000000,
        &vec![&e, 1_500_000_0000000, 40000_0000000],
        &bombadil,
    );

    // deploy a new backstop and backstop token
    let contracts_2 = create_blend_contracts(&e, &bombadil, &usdc_id, &blnd_id);
    contracts
        .backstop_token
        .transfer(&bombadil, &contracts_2.backstop.address, &60_000_0000000);

    contracts.emitter.queue_swap_backstop(
        &contracts_2.backstop.address,
        &contracts_2.backstop_token.address,
    );

    // wait for backstop swap to complete
    e.jump(31 * ONE_DAY_LEDGERS + 1);

    // validate the swap must be complete before the contract can be interacted with
    let lp_mint_amount = 100_0000000;
    let result = blend_lockup_client.try_c_join_pool(
        &contracts_2.backstop_token.address,
        &(lp_mint_amount + 1),
        &vec![&e, 1_100_0000000, 26_0000000],
    );
    assert_eq!(result.err(), Some(Ok(Error::from_contract_error(101))));
    let result = blend_lockup_client.try_b_deposit(
        &contracts_2.backstop.address,
        &contracts_2.backstop_token.address,
        &contracts_2.pool.address,
        &lp_mint_amount,
    );
    assert_eq!(result.err(), Some(Ok(Error::from_contract_error(101))));

    e.mock_all_auths_allowing_non_root_auth();
    contracts.emitter.swap_backstop();
    e.set_auths(&[]);
    e.mock_all_auths();

    // verify lockup update backstop
    assert_eq!(
        blend_lockup_client.backstops(),
        vec![&e, contracts.backstop.address.clone()]
    );
    assert_eq!(
        blend_lockup_client.backstop_tokens(),
        vec![&e, contracts.backstop_token.address.clone()]
    );

    blend_lockup_client.update_backstop();
    assert_eq!(
        e.auths()[0],
        (
            frodo.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    blend_lockup_client.address.clone(),
                    Symbol::new(&e, "update_backstop"),
                    vec![&e,]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    assert_eq!(
        blend_lockup_client.backstops(),
        vec![
            &e,
            contracts.backstop.address.clone(),
            contracts_2.backstop.address.clone()
        ]
    );
    assert_eq!(
        blend_lockup_client.backstop_tokens(),
        vec![
            &e,
            contracts.backstop_token.address.clone(),
            contracts_2.backstop_token.address.clone()
        ]
    );

    // validate both backstop tokens can be interacted with
    let lp_mint_amount = 100_0000000;
    blend_lockup_client.c_join_pool(
        &contracts.backstop_token.address,
        &lp_mint_amount,
        &vec![&e, 1_100_0000000, 26_0000000],
    );
    blend_lockup_client.c_join_pool(
        &contracts_2.backstop_token.address,
        &(lp_mint_amount + 1),
        &vec![&e, 1_100_0000000, 26_0000000],
    );
    assert_eq!(
        lp_mint_amount,
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address)
    );
    assert_eq!(
        lp_mint_amount + 1,
        contracts_2
            .backstop_token
            .balance(&blend_lockup_client.address)
    );

    // validate both backstops can be interacted with
    blend_lockup_client.b_queue_withdrawal(
        &contracts.backstop.address,
        &contracts.pool.address,
        &lp_mint_amount,
    );
    assert_eq!(
        lp_mint_amount,
        contracts
            .backstop
            .user_balance(&contracts.pool.address, &blend_lockup_client.address)
            .q4w
            .get_unchecked(0)
            .amount
    );
    blend_lockup_client.b_deposit(
        &contracts_2.backstop.address,
        &contracts_2.backstop_token.address,
        &contracts_2.pool.address,
        &lp_mint_amount,
    );
    assert_eq!(
        lp_mint_amount,
        contracts_2
            .backstop
            .user_balance(&contracts_2.pool.address, &blend_lockup_client.address)
            .shares
    );
}
