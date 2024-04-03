use crate::{
    dependencies::{
        backstop::Client as BackstopClient, comet::Client as CometClient,
        emitter::Client as EmitterClient,
    },
    errors::BlendLockupError,
    storage,
};
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error,
    token::TokenClient,
    unwrap::UnwrapOptimized,
    vec, Address, Env, IntoVal, Symbol, Vec,
};

#[contract]
pub struct BlendLockup;

#[contractimpl]
impl BlendLockup {
    /********** Constructor **********/

    /// Initialize the contract
    ///
    /// ### Arguments
    /// * owner - The owner of the contract
    /// * emitter - The address of the emitter contract
    /// * unlock - The unlock time (in seconds since epoch)
    ///
    /// ### Errors
    /// * AlreadyInitializedError - The contract has already been initialized
    pub fn initialize(e: Env, owner: Address, emitter: Address, unlock: u64) {
        if storage::get_is_init(&e) {
            panic_with_error!(&e, BlendLockupError::AlreadyInitializedError);
        }
        storage::extend_instance(&e);

        if unlock <= e.ledger().timestamp() {
            panic_with_error!(&e, BlendLockupError::InvalidUnlockTime);
        }
        storage::set_owner(&e, &owner);
        storage::set_emitter(&e, &emitter);
        storage::set_unlock(&e, unlock);

        let backstop = EmitterClient::new(&e, &emitter).get_backstop();
        let backstop_token = BackstopClient::new(&e, &backstop).backstop_token();
        storage::set_valid_backstops(&e, &vec![&e, backstop]);
        storage::set_valid_backstop_tokens(&e, &vec![&e, backstop_token]);

        storage::set_is_init(&e);
    }

    /********** Read-Only **********/

    /// Get owner
    pub fn owner(e: Env) -> Address {
        storage::get_owner(&e)
    }

    /// Get unlock time of the lockup
    pub fn unlock(e: Env) -> u64 {
        storage::get_unlock(&e)
    }

    /// Get the emitter contract
    pub fn emitter(e: Env) -> Address {
        storage::get_emitter(&e)
    }

    /// Get the backstop contracts that have been recorded by the emitter
    pub fn backstops(e: Env) -> Vec<Address> {
        storage::get_valid_backstops(&e)
    }

    /// Get the backstop token contracts that have been recorded by the emitter
    pub fn backstop_tokens(e: Env) -> Vec<Address> {
        storage::get_valid_backstop_tokens(&e)
    }

    /********** Write **********/

    /// (Only Owner) Update the backstop contract and token from the emitter contract.
    pub fn update_backstop(e: Env) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        let emitter = storage::get_emitter(&e);
        let new_backstop = EmitterClient::new(&e, &emitter).get_backstop();
        let new_backstop_token = BackstopClient::new(&e, &new_backstop).backstop_token();

        let mut backstops = storage::get_valid_backstops(&e);
        if !backstops.contains(&new_backstop) {
            backstops.push_back(new_backstop);
            storage::set_valid_backstops(&e, &backstops);
        }
        let mut backstop_tokens = storage::get_valid_backstop_tokens(&e);
        if !backstop_tokens.contains(&new_backstop_token) {
            backstop_tokens.push_back(new_backstop_token);
            storage::set_valid_backstop_tokens(&e, &backstop_tokens);
        }
    }

    /// (Only Owner) Claim assets from the lockup
    ///
    /// ### Arguments
    /// * `assets` - The Vec of addresses of the assets to claim
    ///
    /// ### Errors
    /// * InvalidUnlockTime - The unlock time has not been reached
    pub fn claim(e: Env, assets: Vec<Address>) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        let unlock_time = storage::get_unlock(&e);
        if unlock_time > e.ledger().timestamp() {
            panic_with_error!(&e, BlendLockupError::InvalidUnlockTime);
        }
        for asset in assets.iter() {
            let token_client = TokenClient::new(&e, &asset);
            let balance = token_client.balance(&e.current_contract_address());
            token_client.transfer(&e.current_contract_address(), &owner, &balance);
        }
    }

    /***** Backstop Interactions *****/

    /// (Only Owner) Deposit "amount" backstop tokens from the lockup into the backstop for "pool_address"
    ///
    /// Returns the number of backstop pool shares minted
    ///
    /// ### Arguments
    /// * `backstop` - The address of the backstop contract
    /// * `backstop_token` - The address of the backstop token
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of tokens to deposit
    pub fn b_deposit(
        e: Env,
        backstop: Address,
        backstop_token: Address,
        pool_address: Address,
        amount: i128,
    ) -> i128 {
        storage::get_owner(&e).require_auth();
        storage::extend_instance(&e);

        let backstops = storage::get_valid_backstops(&e);
        let backstop_tokens = storage::get_valid_backstop_tokens(&e);
        if !backstops.contains(&backstop) || !backstop_tokens.contains(&backstop_token) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: backstop_token,
                    fn_name: Symbol::new(&e, "transfer"),
                    args: vec![
                        &e,
                        e.current_contract_address().into_val(&e),
                        backstop.into_val(&e),
                        amount.into_val(&e),
                    ],
                },
                sub_invocations: vec![&e],
            }),
        ]);
        BackstopClient::new(&e, &backstop).deposit(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        )
    }

    /// (Only Owner) Queue deposited pool shares from the lockup for withdraw from a backstop of a pool
    ///
    /// Returns the created queue for withdrawal
    ///
    /// ### Arguments
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to queue for withdraw
    pub fn b_queue_withdrawal(e: Env, backstop: Address, pool_address: Address, amount: i128) {
        storage::get_owner(&e).require_auth();
        storage::extend_instance(&e);

        let backstops = storage::get_valid_backstops(&e);
        if !backstops.contains(&backstop) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        BackstopClient::new(&e, &backstop).queue_withdrawal(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        );
    }

    /// (Only Owner) Dequeue a currently queued pool share withdraw for the lockup from the backstop of a pool
    ///
    /// ### Arguments
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to dequeue
    pub fn b_dequeue_withdrawal(e: Env, backstop: Address, pool_address: Address, amount: i128) {
        storage::get_owner(&e).require_auth();
        storage::extend_instance(&e);

        let backstops = storage::get_valid_backstops(&e);
        if !backstops.contains(&backstop) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        BackstopClient::new(&e, &backstop).dequeue_withdrawal(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        )
    }

    /// (Only Owner) Withdraw shares from the lockup's withdraw queue for a backstop of a pool
    ///
    /// Returns the amount of tokens returned
    ///
    /// ### Arguments
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to withdraw
    pub fn b_withdraw(e: Env, from: Address, pool_address: Address, amount: i128) -> i128 {
        storage::get_owner(&e).require_auth();
        storage::extend_instance(&e);

        let backstops = storage::get_valid_backstops(&e);
        if !backstops.contains(&from) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        BackstopClient::new(&e, &from).withdraw(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        )
    }

    /// (Only Owner) Claim backstop deposit emissions from a list of pools for the lockup
    ///
    /// Returns the amount of BLND emissions claimed
    ///
    /// ### Arguments
    /// * `backstop` - The address of the backstop contract
    /// * `pool_addresses` - The Vec of addresses to claim backstop deposit emissions from
    ///
    /// ### Errors
    /// If an invalid pool address is included
    pub fn b_claim(e: Env, backstop: Address, pool_addresses: Vec<Address>) -> i128 {
        storage::get_owner(&e).require_auth();
        storage::extend_instance(&e);

        let backstops = storage::get_valid_backstops(&e);
        if !backstops.contains(&backstop) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        BackstopClient::new(&e, &backstop).claim(
            &e.current_contract_address(),
            &pool_addresses,
            &e.current_contract_address(),
        )
    }

    /***** Backstop Token Interactions *****/

    /// (Only Owner) Join a backstop token's liquidity pool. Requires that the backstop token implements the Comet interface.
    ///
    /// ### Arguments
    /// * `backstop_token` - The address of the backstop token
    /// * `pool_amount_out` - The amount of pool shares to mint
    /// * `max_amounts_in` - The maximum amount of tokens to deposit
    pub fn c_join_pool(
        e: Env,
        backstop_token: Address,
        pool_amount_out: i128,
        max_amounts_in: Vec<i128>,
    ) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        let backstop_tokens = storage::get_valid_backstop_tokens(&e);
        if !backstop_tokens.contains(&backstop_token) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        let comet = CometClient::new(&e, &backstop_token);
        let comet_tokens = comet.get_tokens();
        let mut auths = vec![&e];
        for index in 0..comet_tokens.len() {
            let amount = max_amounts_in.get(index).unwrap_optimized();
            let token_address = comet_tokens.get(index).unwrap_optimized();
            let approval_ledger = (e.ledger().sequence() / 100000 + 1) * 100000;
            auths.push_back(InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: token_address,
                    fn_name: Symbol::new(&e, "approve"),
                    args: vec![
                        &e,
                        e.current_contract_address().into_val(&e),
                        backstop_token.into_val(&e),
                        amount.into_val(&e),
                        approval_ledger.into_val(&e),
                    ],
                },
                sub_invocations: vec![&e],
            }));
        }
        e.authorize_as_current_contract(auths);
        comet.join_pool(
            &pool_amount_out,
            &max_amounts_in,
            &e.current_contract_address(),
        );
    }

    /// (Only Owner) Exit a backstop token's liquidity pool. Requires that the backstop token implements the Comet interface.
    ///
    /// ### Arguments
    /// * `backstop_token` - The address of the backstop token
    /// * `burn_amount` - The amount of pool shares to burn
    /// * `min_amounts_out` - The minimum amount of tokens to receive
    pub fn c_exit_pool(
        e: Env,
        backstop_token: Address,
        burn_amount: i128,
        min_amounts_out: Vec<i128>,
    ) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        let backstop_tokens = storage::get_valid_backstop_tokens(&e);
        if !backstop_tokens.contains(&backstop_token) {
            panic_with_error!(&e, BlendLockupError::InvalidContractAddress);
        }

        let comet = CometClient::new(&e, &backstop_token);
        comet.exit_pool(
            &burn_amount,
            &min_amounts_out,
            &e.current_contract_address(),
        )
    }
}
