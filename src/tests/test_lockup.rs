#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, Error,
};

use crate::testutils::{
    create_blend_contracts, create_blend_lockup_wasm, EnvTestUtils, ONE_DAY_LEDGERS,
};

#[test]
fn test_lockup() {
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
    );

    // verify initailize can't be run twice
    let result = blend_lockup_client.try_initialize(
        &frodo,
        &contracts.emitter.address,
        &e.ledger().timestamp(),
    );
    assert_eq!(result.err(), Some(Ok(Error::from_contract_error(3))));

    let starting_blnd_balance: i128 = 100_000_0000000;
    let starting_usdc_balance: i128 = 2500_0000000;

    e.jump(ONE_DAY_LEDGERS); // 1 day total
    contracts.emitter.distribute();
    contracts.backstop.gulp_emissions();

    blnd_admin_client.mint(&blend_lockup_client.address, &starting_blnd_balance);
    usdc_admin_client.mint(&blend_lockup_client.address, &starting_usdc_balance);
    let usdc_token = TokenClient::new(&e, &usdc_admin_client.address);
    let blnd_token = TokenClient::new(&e, &blnd_admin_client.address);

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
    let actual_blnd_in = starting_blnd_balance - blnd_token.balance(&blend_lockup_client.address);
    let actual_usdc_in = starting_usdc_balance - usdc_token.balance(&blend_lockup_client.address);

    e.jump(10 * ONE_DAY_LEDGERS); // 11 days total

    blend_lockup_client.b_queue_withdrawal(
        &contracts.backstop.address,
        &contracts.pool.address,
        &(lp_mint_amount / 2),
    );

    // wait for withdrawal queue to expire
    e.jump(22 * ONE_DAY_LEDGERS); // 33 days total

    // claim and add rest of the shares to the queue
    blend_lockup_client.b_claim(
        &contracts.backstop.address,
        &vec![&e, contracts.pool.address.clone()],
    );
    let remaining_shares = contracts
        .backstop
        .user_balance(&contracts.pool.address, &blend_lockup_client.address)
        .shares;
    assert!(remaining_shares > lp_mint_amount / 2);
    blend_lockup_client.b_queue_withdrawal(
        &contracts.backstop.address,
        &contracts.pool.address,
        &remaining_shares,
    );

    // withdraw initially queued shares
    blend_lockup_client.b_withdraw(
        &contracts.backstop.address,
        &contracts.pool.address,
        &(lp_mint_amount / 2),
    );

    assert_eq!(
        lp_mint_amount / 2,
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address)
    );

    // wait until 1 day before unlock period
    e.jump(8 * ONE_DAY_LEDGERS); // 41 days total

    // validate that you cannot claim tokens
    let result = blend_lockup_client.try_claim(&vec![&e, blnd_id.clone()]);
    assert_eq!(result.err(), Some(Ok(Error::from_contract_error(100))));

    // wait until unlock period
    e.jump(1 * ONE_DAY_LEDGERS + 1); // 42 days total
    blend_lockup_client.claim(&vec![
        &e,
        blnd_id.clone(),
        usdc_id.clone(),
        contracts.backstop_token.address.clone(),
    ]);
    assert_eq!(
        0,
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address)
    );
    assert_eq!(0, blnd_token.balance(&blend_lockup_client.address));
    assert_eq!(0, usdc_token.balance(&blend_lockup_client.address));
    assert_eq!(lp_mint_amount / 2, contracts.backstop_token.balance(&frodo));
    assert_eq!(
        starting_blnd_balance - actual_blnd_in,
        blnd_token.balance(&frodo)
    );
    assert_eq!(
        starting_usdc_balance - actual_usdc_in,
        usdc_token.balance(&frodo)
    );

    // wait until we can withdraw the rest of the backstop shares and claim them
    // 9 days have passed since they were queued
    e.jump(12 * ONE_DAY_LEDGERS);
    blend_lockup_client.b_withdraw(
        &contracts.backstop.address,
        &contracts.pool.address,
        &remaining_shares,
    );
    blend_lockup_client.claim(&vec![&e, contracts.backstop_token.address.clone()]);
    assert_eq!(
        lp_mint_amount / 2 + remaining_shares,
        contracts.backstop_token.balance(&frodo)
    );
    assert_eq!(
        0,
        contracts
            .backstop_token
            .balance(&blend_lockup_client.address)
    );
}
