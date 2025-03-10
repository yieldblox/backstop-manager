#![cfg(test)]

use blend_contract_sdk::testutils::BlendFixture;
use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger as _, LedgerInfo},
    Address, BytesN, Env, String,
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
/// * `owner` - The address of the owner
/// * `emitter` - The address of the emitter
/// * `unlock` - The unlock ledger time (in seconds)
pub fn create_backstop_manager_wasm<'a>(
    e: &Env,
    owner: &Address,
    emitter: &Address,
    unlock: &u64,
    bootstrapper: &Address,
) -> (Address, contract::Client<'a>) {
    let token_lockup_address = e.register(contract::WASM, {});
    let token_lockup_client: contract::Client<'a> =
        contract::Client::new(&e, &token_lockup_address);
    token_lockup_client.initialize(owner, emitter, bootstrapper, unlock);
    (token_lockup_address, token_lockup_client)
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
            protocol_version: 20,
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
            protocol_version: 20,
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

    contracts.backstop.deposit(&admin, &pool, &50_000_0000000);
    contracts.backstop.add_reward(&pool, &None);

    (contracts, pool)
}
