use crate::{
    dependencies::{
        bootstrapper::{Bootstrap, BootstrapConfig, Client as BootstrapClient},
        comet::Client as CometClient,
    },
    errors::BackstopManagerErrors,
    storage::{self, Manager},
};
use blend_contract_sdk::backstop::Client as BackstopClient;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    contract, contractimpl, panic_with_error,
    token::TokenClient,
    unwrap::UnwrapOptimized,
    vec, Address, Env, IntoVal, Symbol, Vec,
};

const MAX_VALID_LIST_LEN: u32 = 4;

#[contract]
pub struct BackstopManager;

#[contractimpl]
impl BackstopManager {
    /********** Constructor **********/

    /// Initialize the backstop manager
    ///
    /// ### Arguments
    /// * owner - The address of the owner of the funds
    /// * manager - The address of the manager of the funds
    /// * emitter - The address of the emitter contract
    /// * bootstrapper - The address of the backstop bootstrapper contract
    /// * backstop_token - The address of the backstop token the manager can interact with. This is fixed
    ///                    as the backstop manager only supports the BLND-USDC LP token as the backstop token.
    /// * backstops - The addresses of the backstops the manager can interact with initially
    /// * pools - The addresses of the pools the manager can interact with initially
    ///
    /// ### Errors
    /// * AlreadyInitializedError - The contract has already been initialized
    pub fn __constructor(
        e: Env,
        owner: Address,
        manager: Address,
        admin_scope: u32,
        emitter: Address,
        bootstrapper: Address,
        backstop_token: Address,
        backstops: Vec<Address>,
        pools: Vec<Address>,
    ) {
        storage::set_owner(&e, &owner);
        if admin_scope > 2 {
            panic_with_error!(&e, BackstopManagerErrors::InvalidScope);
        }
        storage::set_manager(
            &e,
            &Manager {
                id: manager,
                scope: admin_scope,
            },
        );
        storage::set_emitter(&e, &emitter);
        storage::set_backstop_bootstrapper(&e, bootstrapper);
        storage::set_backstop_token(&e, backstop_token);

        if backstops.len() > MAX_VALID_LIST_LEN {
            panic_with_error!(&e, BackstopManagerErrors::ContractListOverMax);
        }
        if pools.len() > MAX_VALID_LIST_LEN {
            panic_with_error!(&e, BackstopManagerErrors::ContractListOverMax);
        }
        storage::set_valid_backstops(&e, &backstops);
        storage::set_valid_pools(&e, &pools);
        storage::extend_instance(&e);
    }

    /********** Read-Only **********/

    /// Get owner
    pub fn owner(e: Env) -> Address {
        storage::get_owner(&e)
    }

    /// Get manager
    pub fn manager(e: Env) -> Manager {
        storage::get_manager(&e)
    }

    /// Get the emitter contract
    pub fn emitter(e: Env) -> Address {
        storage::get_emitter(&e)
    }

    /// Get the backstop bootstrapper contract
    pub fn backstop_bootstrapper(e: Env) -> Address {
        storage::get_backstop_bootstrapper(&e)
    }

    /// Get the backstops this contract can interact with
    pub fn backstops(e: Env) -> Vec<Address> {
        storage::get_valid_backstops(&e)
    }

    /// Get the pools this contract can interact with
    pub fn pools(e: Env) -> Vec<Address> {
        storage::get_valid_pools(&e)
    }

    /********** Owner **********/

    /// (Only Owner) Transfer tokens from the manager contract to another address
    ///
    /// ### Arguments
    /// * `token` - The address of the token to transfer
    /// * `to` - The address to transfer the tokens to
    /// * `amount` - The amount of tokens to transfer
    pub fn transfer_token(e: Env, token: Address, to: Address, amount: i128) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        let token_client = TokenClient::new(&e, &token);
        token_client.transfer(&e.current_contract_address(), &to, &amount);
    }

    /// (Only Owner) Set the manager for the contract
    ///
    /// ### Arguments
    /// * `manager` - The address of the manager
    /// * `scope` - The scope of the manager. 0 = None, 1 = Low, 2 = High
    pub fn set_manager(e: Env, manager: Address, scope: u32) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        if scope > 2 {
            panic_with_error!(&e, BackstopManagerErrors::InvalidScope);
        }

        storage::set_manager(&e, &Manager { id: manager, scope });
    }

    /// (Only Owner) Set the backstop bootstrapper contract
    ///
    /// ### Arguments
    /// * `bootstrapper` - The address of the backstop bootstrapper contract
    pub fn set_backstop_bootstrapper(e: Env, bootstrapper: Address) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);

        storage::set_backstop_bootstrapper(&e, bootstrapper);
    }

    /// (Only Owner) Set the list of valid backstops
    ///
    /// ### Arguments
    /// * `backstops` - The addresses of the backstops the manager can interact with
    pub fn set_backstops(e: Env, backstops: Vec<Address>) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);
        if backstops.len() > MAX_VALID_LIST_LEN {
            panic_with_error!(&e, BackstopManagerErrors::ContractListOverMax);
        }
        storage::set_valid_backstops(&e, &backstops);
    }

    /// (Only Owner) Set the list of valid pools
    ///
    /// ### Arguments
    /// * `pools` - The addresses of the backstops the manager can interact with
    pub fn set_pools(e: Env, pools: Vec<Address>) {
        let owner = storage::get_owner(&e);
        owner.require_auth();
        storage::extend_instance(&e);
        if pools.len() > MAX_VALID_LIST_LEN {
            panic_with_error!(&e, BackstopManagerErrors::ContractListOverMax);
        }
        storage::set_valid_pools(&e, &pools);
    }

    /********** Manager **********/

    /// (Manager, Low) Transfer tokens from the contract back to the owner
    ///
    /// ### Arguments
    /// * `token` - The address of the token to transfer
    /// * `to` - The address to transfer the tokens to
    /// * `amount` - The amount of tokens to transfer
    pub fn refund_token(e: Env, from: Address, token: Address, amount: i128) {
        require_auth_with_scope(&e, from, 0);
        storage::extend_instance(&e);

        let owner = storage::get_owner(&e);
        let token_client = TokenClient::new(&e, &token);
        token_client.transfer(&e.current_contract_address(), &owner, &amount);
    }

    /***** Backstop Interactions *****/

    /// (Manager, Low) Claim backstop deposit emissions from a list of pools for the contract
    ///
    /// Returns the amount of BLND emissions claimed
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool to claim from
    ///
    /// ### Errors
    /// If an invalid pool address is included
    pub fn b_claim(e: Env, from: Address, backstop: Address, pool_address: Address) -> i128 {
        require_auth_with_scope(&e, from, 0);
        require_backstop_and_pool_valid(&e, &backstop, &pool_address);
        storage::extend_instance(&e);

        BackstopClient::new(&e, &backstop).claim(
            &e.current_contract_address(),
            &vec![&e, pool_address],
            &e.current_contract_address(),
        )
    }

    /// (Manger, Medium) Deposit "amount" backstop tokens from the contract into the backstop for "pool_address"
    ///
    /// Returns the number of backstop pool shares minted
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of tokens to deposit
    pub fn b_deposit(
        e: Env,
        from: Address,
        backstop: Address,
        pool_address: Address,
        amount: i128,
    ) -> i128 {
        require_auth_with_scope(&e, from, 1);
        require_backstop_and_pool_valid(&e, &backstop, &pool_address);
        storage::extend_instance(&e);

        let backstop_token = storage::get_backstop_token(&e);
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

    /// (Manager, Medium) Queue deposited pool shares from the contract for withdraw from a backstop of a pool
    ///
    /// Returns the created queue for withdrawal
    ///
    /// ### Arguments
    /// * `from` - The address of the backstop contract
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to queue for withdraw
    pub fn b_queue_withdrawal(
        e: Env,
        from: Address,
        backstop: Address,
        pool_address: Address,
        amount: i128,
    ) {
        require_auth_with_scope(&e, from, 1);
        require_backstop_and_pool_valid(&e, &backstop, &pool_address);
        storage::extend_instance(&e);

        BackstopClient::new(&e, &backstop).queue_withdrawal(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        );
    }

    /// (Manager, Medium) Dequeue a currently queued pool share withdraw for the contract from the backstop of a pool
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to dequeue
    pub fn b_dequeue_withdrawal(
        e: Env,
        from: Address,
        backstop: Address,
        pool_address: Address,
        amount: i128,
    ) {
        require_auth_with_scope(&e, from, 1);
        require_backstop_and_pool_valid(&e, &backstop, &pool_address);
        storage::extend_instance(&e);

        BackstopClient::new(&e, &backstop).dequeue_withdrawal(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        )
    }

    /// (Manager, High) Withdraw shares from the contract's withdraw queue for a backstop of a pool
    ///
    /// Returns the amount of tokens returned
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `backstop` - The address of the backstop contract
    /// * `pool_address` - The address of the pool
    /// * `amount` - The amount of shares to withdraw
    pub fn b_withdraw(
        e: Env,
        from: Address,
        backstop: Address,
        pool_address: Address,
        amount: i128,
    ) -> i128 {
        require_auth_with_scope(&e, from, 2);
        require_backstop_and_pool_valid(&e, &backstop, &pool_address);
        storage::extend_instance(&e);

        BackstopClient::new(&e, &backstop).withdraw(
            &e.current_contract_address(),
            &pool_address,
            &amount,
        )
    }

    /***** Backstop Token Interactions *****/

    /// (Manager, High) Join the BLND-USDC LP.
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `backstop_token` - The address of the backstop token
    /// * `pool_amount_out` - The amount of pool shares to mint
    /// * `max_amounts_in` - The maximum amount of tokens to deposit
    pub fn c_join_pool(e: Env, from: Address, pool_amount_out: i128, max_amounts_in: Vec<i128>) {
        require_auth_with_scope(&e, from, 2);
        storage::extend_instance(&e);

        let backstop_token = storage::get_backstop_token(&e);
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

    /// (Manager, High) Exit a backstop token's liquidity pool.
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `burn_amount` - The amount of pool shares to burn
    /// * `min_amounts_out` - The minimum amount of tokens to receive
    pub fn c_exit_pool(e: Env, from: Address, burn_amount: i128, min_amounts_out: Vec<i128>) {
        require_auth_with_scope(&e, from, 2);
        storage::extend_instance(&e);

        let backstop_token = storage::get_backstop_token(&e);
        let comet = CometClient::new(&e, &backstop_token);
        comet.exit_pool(
            &burn_amount,
            &min_amounts_out,
            &e.current_contract_address(),
        )
    }

    /***** Backstop Bootstrapper Interactions *****/

    /// (Manager, Low) Claims the proceeds of a backstop bootstrapping
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `bootstrap_id` - The id of the bootstrapper
    /// * `backstop` - The address of the backstop the bootstrap is for
    pub fn bb_claim_bootstrap(e: Env, from: Address, bootstrap_id: u32, backstop: Address) {
        require_auth_with_scope(&e, from, 0);
        storage::extend_instance(&e);

        // no need to validate backstop arg, as it's just used to pre-auth the deposit, and is never
        // invoked directly. If an invalid arg is passed, the pre-auth will be invalid.

        let backstop_bootstrapper_client =
            BootstrapClient::new(&e, &storage::get_backstop_bootstrapper(&e));
        let bootstrap: Bootstrap = backstop_bootstrapper_client.get_bootstrap(&bootstrap_id);
        let comet_client = CometClient::new(&e, &storage::get_backstop_token(&e));

        let backstop_token_amount = bootstrap.data.total_backstop_tokens
            * (comet_client.get_normalized_weight(
                &comet_client
                    .get_tokens()
                    .get(bootstrap.config.token_index)
                    .unwrap_optimized(),
            ) as i128)
            / 1_000_0000;

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: backstop.clone(),
                    fn_name: Symbol::new(&e, "deposit"),
                    args: vec![
                        &e,
                        e.current_contract_address().into_val(&e),
                        bootstrap.config.pool.into_val(&e),
                        backstop_token_amount.into_val(&e),
                    ],
                },
                sub_invocations: vec![
                    &e,
                    InvokerContractAuthEntry::Contract(SubContractInvocation {
                        context: ContractContext {
                            contract: comet_client.address,
                            fn_name: Symbol::new(&e, "transfer"),
                            args: vec![
                                &e,
                                e.current_contract_address().into_val(&e),
                                backstop.into_val(&e),
                                backstop_token_amount.into_val(&e),
                            ],
                        },
                        sub_invocations: vec![&e],
                    }),
                ],
            }),
        ]);

        backstop_bootstrapper_client.claim(&e.current_contract_address(), &bootstrap_id);
    }

    /// (Manager, Low) Refunds a cancelled backstop bootstrapping
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `bootstrap_id` - The id of the bootstrapper
    pub fn bb_refund_bootstrap(e: Env, from: Address, bootstrap_id: u32) {
        require_auth_with_scope(&e, from, 0);
        storage::extend_instance(&e);

        let backstop_bootstrapper_client =
            BootstrapClient::new(&e, &storage::get_backstop_bootstrapper(&e));

        backstop_bootstrapper_client.refund(&e.current_contract_address(), &bootstrap_id);
    }

    /// (Manager, High) Creates a Backstop Bootstrapping with BLND
    ///
    /// ### Arguments
    /// * `from` - The caller of the function
    /// * `bootstrap_token` - The address of the bootstrap token
    /// * `bootstrap_amount` - The amount of tokens to bootstrap
    /// * `pair_min` - The minimum amount of pool shares to mint
    /// * `duration` - The duration of the bootstrapping period
    /// * `pool_address` - The address of the pool
    pub fn bb_start_bootstrap(
        e: Env,
        from: Address,
        bootstrap_token_index: u32,
        bootstrap_amount: i128,
        pair_min: i128,
        duration: u32,
        pool_address: Address,
    ) {
        require_auth_with_scope(&e, from, 2);
        storage::extend_instance(&e);

        let backstop_token = storage::get_backstop_token(&e);
        let bootstrap_token: Address = match CometClient::new(&e, &backstop_token)
            .get_tokens()
            .get(bootstrap_token_index)
        {
            Some(address) => address,
            None => panic_with_error!(e, BackstopManagerErrors::InvalidTokenIndex),
        };

        let backstop_bootstrapper = storage::get_backstop_bootstrapper(&e);
        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract(SubContractInvocation {
                context: ContractContext {
                    contract: bootstrap_token,
                    fn_name: Symbol::new(&e, "transfer"),
                    args: vec![
                        &e,
                        e.current_contract_address().into_val(&e),
                        backstop_bootstrapper.into_val(&e),
                        bootstrap_amount.into_val(&e),
                    ],
                },
                sub_invocations: vec![&e],
            }),
        ]);

        BootstrapClient::new(&e, &backstop_bootstrapper).bootstrap(&BootstrapConfig {
            bootstrapper: e.current_contract_address(),
            amount: bootstrap_amount,
            close_ledger: e.ledger().sequence() + duration,
            pair_min,
            pool: pool_address,
            token_index: bootstrap_token_index,
        });
    }
}

/// Authorize an action based on a provide scope for from. If `from` is the owner,
/// then the action is authorized. If `from` is the manager, then the manager is validated
/// to have the appropriate scope.
///
/// THIS CALLS REQUIRE AUTH FOR FROM
///
/// ### Arguments
/// * `from` - The address of the caller
/// * `scope` - The scope required for the action
///
/// ### Errors
/// * UnauthorizedError - The caller is not authorized to perform the action
fn require_auth_with_scope(e: &Env, from: Address, scope: u32) {
    from.require_auth();
    if from == storage::get_owner(&e) {
        return;
    }
    let manager = storage::get_manager(&e);
    if manager.id != from || manager.scope < scope {
        panic_with_error!(&e, BackstopManagerErrors::UnauthorizedError);
    }
}

/// Validate that the backstop and pool address are included in the valid lists
///
/// ### Arguments
/// * `backstop` - The address of the backstop contract
/// * `pool_address` - The address of the pool
///
/// ### Errors
/// * InvalidContractAddress - The backstop or pool address is not included in the valid lists
fn require_backstop_and_pool_valid(e: &Env, backstop: &Address, pool_address: &Address) {
    let backstops = storage::get_valid_backstops(&e);
    let pools = storage::get_valid_pools(&e);
    if !backstops.contains(backstop) || !pools.contains(pool_address) {
        panic_with_error!(&e, BackstopManagerErrors::InvalidContractAddress);
    }
}
