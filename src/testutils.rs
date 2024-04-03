#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger as _, LedgerInfo},
    token::StellarAssetClient,
    vec, Address, BytesN, Env, Map, Symbol,
};

use crate::dependencies::{backstop, comet, emitter, pool, pool_factory};

mod contract {
    soroban_sdk::contractimport!(
        file = "./target/wasm32-unknown-unknown/optimized/blend_lockup.wasm"
    );
}

/// Create a blend lockup contract via wasm
///
/// ### Arguments
/// * `owner` - The address of the owner
/// * `emitter` - The address of the emitter
/// * `unlock` - The unlock ledger time (in seconds)
pub fn create_blend_lockup_wasm<'a>(
    e: &Env,
    owner: &Address,
    emitter: &Address,
    unlock: &u64,
) -> (Address, contract::Client<'a>) {
    let token_lockup_address = e.register_contract_wasm(None, contract::WASM);
    let token_lockup_client: contract::Client<'a> =
        contract::Client::new(&e, &token_lockup_address);
    token_lockup_client.initialize(owner, emitter, unlock);
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

pub struct BlendContracts<'a> {
    pub backstop: backstop::Client<'a>,
    pub emitter: emitter::Client<'a>,
    pub backstop_token: comet::Client<'a>,
    pub pool_factory: pool_factory::Client<'a>,
    pub pool: pool::Client<'a>,
}

pub fn create_blend_contracts<'a>(
    e: &Env,
    admin: &Address,
    usdc: &Address,
    blnd: &Address,
) -> BlendContracts<'a> {
    let backstop = e.register_contract_wasm(None, backstop::WASM);
    let emitter = e.register_contract_wasm(None, emitter::WASM);
    let comet = e.register_contract_wasm(None, comet::WASM);
    let pool_factory = e.register_contract_wasm(None, pool_factory::WASM);
    let usdc_client = StellarAssetClient::new(&e, &usdc);
    let blnd_client = StellarAssetClient::new(&e, &blnd);
    blnd_client.mint(&admin, &1_000_0000000);
    usdc_client.mint(&admin, &25_0000000);

    let comet_client: comet::Client<'a> = comet::Client::new(&e, &comet);
    comet_client.init(&Address::generate(&e), &admin);

    comet_client.bundle_bind(
        &vec![&e, blnd.clone(), usdc.clone()],
        &vec![&e, 1_000_0000000, 25_0000000],
        &vec![&e, 8_0000000, 2_0000000],
    );
    comet_client.set_swap_fee(&30000_i128, &admin);
    comet_client.set_public_swap(&admin, &true);
    comet_client.finalize();

    blnd_client.set_admin(&emitter);
    let emitter_client: emitter::Client<'a> = emitter::Client::new(&e, &emitter);
    emitter_client.initialize(&blnd, &backstop, &comet);

    let backstop_client: backstop::Client<'a> = backstop::Client::new(&e, &backstop);
    backstop_client.initialize(&comet, &emitter, &usdc, &blnd, &pool_factory, &Map::new(&e));

    let pool_hash = e.deployer().upload_contract_wasm(pool::WASM);

    let pool_factory_client = pool_factory::Client::new(&e, &pool_factory);
    pool_factory_client.initialize(&pool_factory::PoolInitMeta {
        backstop,
        blnd_id: blnd.clone(),
        pool_hash,
    });

    let pool_address = pool_factory_client.deploy(
        &admin,
        &Symbol::new(&e, "test"),
        &BytesN::<32>::random(&e),
        &Address::generate(&e),
        &1000000,
        &6,
    );
    blnd_client.mint(&admin, &1_001_000_0000000);
    usdc_client.mint(&admin, &25000_0000000);
    comet_client.join_pool(
        &50_000_0000000,
        &vec![&e, 1_001_000_0000000, 25000_0000000],
        &admin,
    );

    backstop_client.deposit(&admin, &pool_address, &50_000_0000000);
    backstop_client.update_tkn_val();
    backstop_client.add_reward(&pool_address, &Address::generate(&e));
    BlendContracts {
        backstop: backstop_client,
        emitter: emitter_client,
        backstop_token: comet_client,
        pool_factory: pool_factory_client,
        pool: pool::Client::new(&e, &pool_address),
    }
}
