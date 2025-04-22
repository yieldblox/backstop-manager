#![cfg(test)]

use blend_contract_sdk::{pool::Client as PoolClient, testutils::BlendFixture};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger as _, LedgerInfo},
    Address, BytesN, Env, String, Vec,
};

use crate::dependencies::bootstrapper;

mod contract {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/optimized/backstop_manager.wasm"
    );
}

/// Create a backstop manager contract via wasm
///
/// ### Arguments
/// * owner - The address of the owner of the funds
/// * manager - The address of the manager of the funds
/// * bootstrapper - The address of the backstop bootstrapper contract
/// * backstop_token - The address of the backstop token the manager can interact with. This is fixed
///                    as the backstop manager only supports the BLND-USDC LP token as the backstop token.
/// * backstops - The addresses of the backstops the manager can interact with initially
/// * pools - The addresses of the pools the manager can interact with initially
pub fn create_backstop_manager_wasm<'a>(
    e: &Env,
    owner: &Address,
    manager: &Address,
    admin_scope: &u32,
    bootstrapper: &Address,
    backstop_token: &Address,
    backstops: &Vec<Address>,
    pools: &Vec<Address>,
) -> (Address, contract::Client<'a>) {
    let backstop_manager_address = e.register(
        contract::WASM,
        (
            owner,
            manager,
            admin_scope,
            bootstrapper,
            backstop_token,
            backstops.clone(),
            pools.clone(),
        ),
    );
    let backstop_manager_client: contract::Client<'a> =
        contract::Client::new(&e, &backstop_manager_address);
    (backstop_manager_address, backstop_manager_client)
}

/***** Env Utils *****/

pub const ONE_DAY_LEDGERS: u32 = 17280;

pub trait EnvTestUtils {
    /// Jump the env by the given amount of ledgers. Assumes 5 seconds per ledger.
    fn jump(&self, ledgers: u32);

    /// Set the ledger to the default LedgerInfo
    ///
    /// Time -> 1441065600 (Sept 1st, 2015 12:00:00 AM UTC)
    /// Sequence -> 100
    fn set_default_info(&self);
}

impl EnvTestUtils for Env {
    fn jump(&self, ledgers: u32) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(ledgers as u64 * 5),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(ledgers),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 120 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 120 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 120 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 120 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }
}

/***** Blend Utils *****/

pub fn create_backstop_bootstrapper<'a>(
    e: &Env,
    blend_fixture: &BlendFixture,
) -> bootstrapper::Client<'a> {
    let backstop_bootstrapper = e.register(bootstrapper::WASM, {});
    let backstop_bootstrapper_client = bootstrapper::Client::new(&e, &backstop_bootstrapper);
    backstop_bootstrapper_client.initialize(
        &blend_fixture.backstop.address,
        &blend_fixture.backstop_token.address,
        &blend_fixture.pool_factory.address,
    );
    backstop_bootstrapper_client
}

pub fn create_blend_contracts<'a>(
    e: &Env,
    admin: &Address,
    blnd_id: &Address,
    usdc_id: &Address,
) -> (BlendFixture<'a>, Address) {
    let contracts = BlendFixture::deploy(&e, &admin, &blnd_id, &usdc_id);
    let pool = contracts.pool_factory.deploy(
        &admin,
        &String::from_str(&e, "test"),
        &BytesN::<32>::random(&e),
        &Address::generate(&e),
        &0,
        &2,
        &0,
    );
    let pool_client = PoolClient::new(&e, &pool);

    contracts.backstop.deposit(&admin, &pool, &50_000_0000000);
    contracts.backstop.add_reward(&pool, &None);

    // initialize emissions
    contracts.emitter.distribute();
    contracts.backstop.distribute();

    e.jump(7 * ONE_DAY_LEDGERS);

    // emit 7 days worth of emissions
    contracts.emitter.distribute();
    contracts.backstop.distribute();
    pool_client.gulp_emissions();

    e.jump(1);

    (contracts, pool)
}
