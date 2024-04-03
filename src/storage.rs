use soroban_sdk::{unwrap::UnwrapOptimized, Address, Env, Symbol, Vec};

/********** Ledger Thresholds **********/

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5 seconds per ledger

const LEDGER_BUMP: u32 = 120 * ONE_DAY_LEDGERS;
const LEDGER_THRESHOLD: u32 = LEDGER_BUMP - 20 * ONE_DAY_LEDGERS;

/********** Ledger Keys **********/

const OWNER_KEY: &str = "Owner";
const EMITTER_KEY: &str = "Emit";
const BACKSTOP_KEY: &str = "Bstop";
const BACKSTOP_TOKEN_KEY: &str = "BstopTkn";
const IS_INIT_KEY: &str = "IsInit";
const UNLOCK_KEY: &str = "Unlock";

/********** Ledger Thresholds **********/

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_BUMP);
}

/********** Instance **********/

/// Check if the contract has been initialized
pub fn get_is_init(e: &Env) -> bool {
    e.storage().instance().has(&Symbol::new(e, IS_INIT_KEY))
}

/// Set the contract as initialized
pub fn set_is_init(e: &Env) {
    e.storage()
        .instance()
        .set::<Symbol, bool>(&Symbol::new(e, IS_INIT_KEY), &true);
}

/// Get the owner address
pub fn get_owner(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, OWNER_KEY))
        .unwrap()
}

/// Set the owner address
pub fn set_owner(e: &Env, owner: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, OWNER_KEY), &owner);
}

/// Get the emitter address
pub fn get_emitter(e: &Env) -> Address {
    e.storage()
        .instance()
        .get::<Symbol, Address>(&Symbol::new(e, EMITTER_KEY))
        .unwrap()
}

/// Set the emitter address
pub fn set_emitter(e: &Env, emitter: &Address) {
    e.storage()
        .instance()
        .set::<Symbol, Address>(&Symbol::new(e, EMITTER_KEY), &emitter);
}

/// Get the time of the lockup unlock
pub fn get_unlock(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get::<Symbol, u64>(&Symbol::new(e, UNLOCK_KEY))
        .unwrap_or(0_u64)
}

/// Set the mapping of sequence to unlock percentage
pub fn set_unlock(e: &Env, unlock: u64) {
    e.storage()
        .instance()
        .set::<Symbol, u64>(&Symbol::new(e, UNLOCK_KEY), &unlock);
}

/********** Persistant **********/

/// Get an array of all valid backstops the emitter has recorded
pub fn get_valid_backstops(e: &Env) -> Vec<Address> {
    let key = Symbol::new(e, BACKSTOP_KEY);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
    e.storage()
        .persistent()
        .get::<Symbol, Vec<Address>>(&key)
        .unwrap_optimized()
}

/// Set the valid backstop addresses
pub fn set_valid_backstops(e: &Env, backstop: &Vec<Address>) {
    let key = Symbol::new(e, BACKSTOP_KEY);
    e.storage()
        .persistent()
        .set::<Symbol, Vec<Address>>(&key, &backstop);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}

/// Get the backstop token address
pub fn get_valid_backstop_tokens(e: &Env) -> Vec<Address> {
    let key = Symbol::new(e, BACKSTOP_TOKEN_KEY);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
    e.storage()
        .persistent()
        .get::<Symbol, Vec<Address>>(&key)
        .unwrap_optimized()
}

/// Set the backstop token address
pub fn set_valid_backstop_tokens(e: &Env, backstop_token: &Vec<Address>) {
    let key = Symbol::new(e, BACKSTOP_TOKEN_KEY);
    e.storage()
        .persistent()
        .set::<Symbol, Vec<Address>>(&key, &backstop_token);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}
