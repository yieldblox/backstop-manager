import { Buffer } from "buffer";
import {
  AssembledTransaction,
  Client as ContractClient,
  ClientOptions as ContractClientOptions,
  MethodOptions,
  Spec as ContractSpec,
} from '@stellar/stellar-sdk/contract';
import type {
  u32,
  i128,
} from '@stellar/stellar-sdk/contract';
export * from '@stellar/stellar-sdk'
export * as contract from '@stellar/stellar-sdk/contract'
export * as rpc from '@stellar/stellar-sdk/rpc'

if (typeof window !== 'undefined') {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}

/**
 * The error codes for the contract.
 */
export const Errors = {
  1: {message:"InternalError"},

  3: {message:"AlreadyInitializedError"},

  4: {message:"UnauthorizedError"},

  8: {message:"NegativeAmountError"},

  9: {message:"AllowanceError"},

  10: {message:"BalanceError"},

  12: {message:"OverflowError"},

  100: {message:"ContractListOverMax"},

  101: {message:"InvalidContractAddress"},

  102: {message:"InvalidScope"},

  103: {message:"InvalidTokenIndex"}
}

export interface Manager {
  /**
 * The address of the manager
 */
id: string;
  /**
 * The scope of the manager
 * 0 -> Low
 * 1 -> Medium
 * 2 -> High
 */
scope: u32;
}


export interface Client {
  /**
   * Construct and simulate a owner transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get owner
   */
  owner: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a manager transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get manager
   */
  manager: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Manager>>

  /**
   * Construct and simulate a emitter transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the emitter contract
   */
  emitter: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a backstop_bootstrapper transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the backstop bootstrapper contract
   */
  backstop_bootstrapper: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<string>>

  /**
   * Construct and simulate a backstops transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the backstops this contract can interact with
   */
  backstops: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<string>>>

  /**
   * Construct and simulate a pools transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * Get the pools this contract can interact with
   */
  pools: (options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<Array<string>>>

  /**
   * Construct and simulate a transfer_token transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Only Owner) Transfer tokens from the manager contract to another address
   * 
   * ### Arguments
   * * `token` - The address of the token to transfer
   * * `to` - The address to transfer the tokens to
   * * `amount` - The amount of tokens to transfer
   */
  transfer_token: ({token, to, amount}: {token: string, to: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_manager transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Only Owner) Set the manager for the contract
   * 
   * ### Arguments
   * * `manager` - The address of the manager
   * * `scope` - The scope of the manager. 0 = None, 1 = Low, 2 = High
   */
  set_manager: ({manager, scope}: {manager: string, scope: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_backstop_bootstrapper transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Only Owner) Set the backstop bootstrapper contract
   * 
   * ### Arguments
   * * `bootstrapper` - The address of the backstop bootstrapper contract
   */
  set_backstop_bootstrapper: ({bootstrapper}: {bootstrapper: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_backstops transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Only Owner) Set the list of valid backstops
   * 
   * ### Arguments
   * * `backstops` - The addresses of the backstops the manager can interact with
   */
  set_backstops: ({backstops}: {backstops: Array<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a set_pools transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Only Owner) Set the list of valid pools
   * 
   * ### Arguments
   * * `pools` - The addresses of the backstops the manager can interact with
   */
  set_pools: ({pools}: {pools: Array<string>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a refund_token transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Low) Transfer tokens from the contract back to the owner
   * 
   * ### Arguments
   * * `token` - The address of the token to transfer
   * * `to` - The address to transfer the tokens to
   * * `amount` - The amount of tokens to transfer
   */
  refund_token: ({from, token, amount}: {from: string, token: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a b_claim transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Low) Claim backstop deposit emissions from a list of pools for the contract
   * 
   * Returns the amount of BLND emissions claimed
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `backstop` - The address of the backstop contract
   * * `pool_address` - The address of the pool to claim from
   * 
   * ### Errors
   * If an invalid pool address is included
   */
  b_claim: ({from, backstop, pool_address}: {from: string, backstop: string, pool_address: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a b_deposit transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manger, Medium) Deposit "amount" backstop tokens from the contract into the backstop for "pool_address"
   * 
   * Returns the number of backstop pool shares minted
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `backstop` - The address of the backstop contract
   * * `pool_address` - The address of the pool
   * * `amount` - The amount of tokens to deposit
   */
  b_deposit: ({from, backstop, pool_address, amount}: {from: string, backstop: string, pool_address: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a b_queue_withdrawal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Medium) Queue deposited pool shares from the contract for withdraw from a backstop of a pool
   * 
   * Returns the created queue for withdrawal
   * 
   * ### Arguments
   * * `from` - The address of the backstop contract
   * * `backstop` - The address of the backstop contract
   * * `pool_address` - The address of the pool
   * * `amount` - The amount of shares to queue for withdraw
   */
  b_queue_withdrawal: ({from, backstop, pool_address, amount}: {from: string, backstop: string, pool_address: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a b_dequeue_withdrawal transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Medium) Dequeue a currently queued pool share withdraw for the contract from the backstop of a pool
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `backstop` - The address of the backstop contract
   * * `pool_address` - The address of the pool
   * * `amount` - The amount of shares to dequeue
   */
  b_dequeue_withdrawal: ({from, backstop, pool_address, amount}: {from: string, backstop: string, pool_address: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a b_withdraw transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, High) Withdraw shares from the contract's withdraw queue for a backstop of a pool
   * 
   * Returns the amount of tokens returned
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `backstop` - The address of the backstop contract
   * * `pool_address` - The address of the pool
   * * `amount` - The amount of shares to withdraw
   */
  b_withdraw: ({from, backstop, pool_address, amount}: {from: string, backstop: string, pool_address: string, amount: i128}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<i128>>

  /**
   * Construct and simulate a c_join_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, High) Join the BLND-USDC LP.
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `backstop_token` - The address of the backstop token
   * * `pool_amount_out` - The amount of pool shares to mint
   * * `max_amounts_in` - The maximum amount of tokens to deposit
   */
  c_join_pool: ({from, pool_amount_out, max_amounts_in}: {from: string, pool_amount_out: i128, max_amounts_in: Array<i128>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a c_exit_pool transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, High) Exit a backstop token's liquidity pool.
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `burn_amount` - The amount of pool shares to burn
   * * `min_amounts_out` - The minimum amount of tokens to receive
   */
  c_exit_pool: ({from, burn_amount, min_amounts_out}: {from: string, burn_amount: i128, min_amounts_out: Array<i128>}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a bb_claim_bootstrap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Low) Claims the proceeds of a backstop bootstrapping
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `bootstrap_id` - The id of the bootstrapper
   * * `backstop` - The address of the backstop the bootstrap is for
   */
  bb_claim_bootstrap: ({from, bootstrap_id, backstop}: {from: string, bootstrap_id: u32, backstop: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a bb_refund_bootstrap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, Low) Refunds a cancelled backstop bootstrapping
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `bootstrap_id` - The id of the bootstrapper
   */
  bb_refund_bootstrap: ({from, bootstrap_id}: {from: string, bootstrap_id: u32}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

  /**
   * Construct and simulate a bb_start_bootstrap transaction. Returns an `AssembledTransaction` object which will have a `result` field containing the result of the simulation. If this transaction changes contract state, you will need to call `signAndSend()` on the returned object.
   * (Manager, High) Creates a Backstop Bootstrapping with BLND
   * 
   * ### Arguments
   * * `from` - The caller of the function
   * * `bootstrap_token` - The address of the bootstrap token
   * * `bootstrap_amount` - The amount of tokens to bootstrap
   * * `pair_min` - The minimum amount of pool shares to mint
   * * `duration` - The duration of the bootstrapping period
   * * `pool_address` - The address of the pool
   */
  bb_start_bootstrap: ({from, bootstrap_token_index, bootstrap_amount, pair_min, duration, pool_address}: {from: string, bootstrap_token_index: u32, bootstrap_amount: i128, pair_min: i128, duration: u32, pool_address: string}, options?: {
    /**
     * The fee to pay for the transaction. Default: BASE_FEE
     */
    fee?: number;

    /**
     * The maximum amount of time to wait for the transaction to complete. Default: DEFAULT_TIMEOUT
     */
    timeoutInSeconds?: number;

    /**
     * Whether to automatically simulate the transaction when constructing the AssembledTransaction. Default: true
     */
    simulate?: boolean;
  }) => Promise<AssembledTransaction<null>>

}
export class Client extends ContractClient {
  static async deploy<T = Client>(
        /** Constructor/Initialization Args for the contract's `__constructor` method */
        {owner, manager, admin_scope, emitter, bootstrapper, backstop_token, backstops, pools}: {owner: string, manager: string, admin_scope: u32, emitter: string, bootstrapper: string, backstop_token: string, backstops: Array<string>, pools: Array<string>},
    /** Options for initalizing a Client as well as for calling a method, with extras specific to deploying. */
    options: MethodOptions &
      Omit<ContractClientOptions, "contractId"> & {
        /** The hash of the Wasm blob, which must already be installed on-chain. */
        wasmHash: Buffer | string;
        /** Salt used to generate the contract's ID. Passed through to {@link Operation.createCustomContract}. Default: random. */
        salt?: Buffer | Uint8Array;
        /** The format used to decode `wasmHash`, if it's provided as a string. */
        format?: "hex" | "base64";
      }
  ): Promise<AssembledTransaction<T>> {
    return ContractClient.deploy({owner, manager, admin_scope, emitter, bootstrapper, backstop_token, backstops, pools}, options)
  }
  constructor(public readonly options: ContractClientOptions) {
    super(
      new ContractSpec([ "AAAAAAAAAq9Jbml0aWFsaXplIHRoZSBiYWNrc3RvcCBtYW5hZ2VyCgojIyMgQXJndW1lbnRzCiogb3duZXIgLSBUaGUgYWRkcmVzcyBvZiB0aGUgb3duZXIgb2YgdGhlIGZ1bmRzCiogbWFuYWdlciAtIFRoZSBhZGRyZXNzIG9mIHRoZSBtYW5hZ2VyIG9mIHRoZSBmdW5kcwoqIGVtaXR0ZXIgLSBUaGUgYWRkcmVzcyBvZiB0aGUgZW1pdHRlciBjb250cmFjdAoqIGJvb3RzdHJhcHBlciAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBib290c3RyYXBwZXIgY29udHJhY3QKKiBiYWNrc3RvcF90b2tlbiAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCB0b2tlbiB0aGUgbWFuYWdlciBjYW4gaW50ZXJhY3Qgd2l0aC4gVGhpcyBpcyBmaXhlZAphcyB0aGUgYmFja3N0b3AgbWFuYWdlciBvbmx5IHN1cHBvcnRzIHRoZSBCTE5ELVVTREMgTFAgdG9rZW4gYXMgdGhlIGJhY2tzdG9wIHRva2VuLgoqIGJhY2tzdG9wcyAtIFRoZSBhZGRyZXNzZXMgb2YgdGhlIGJhY2tzdG9wcyB0aGUgbWFuYWdlciBjYW4gaW50ZXJhY3Qgd2l0aCBpbml0aWFsbHkKKiBwb29scyAtIFRoZSBhZGRyZXNzZXMgb2YgdGhlIHBvb2xzIHRoZSBtYW5hZ2VyIGNhbiBpbnRlcmFjdCB3aXRoIGluaXRpYWxseQoKIyMjIEVycm9ycwoqIEFscmVhZHlJbml0aWFsaXplZEVycm9yIC0gVGhlIGNvbnRyYWN0IGhhcyBhbHJlYWR5IGJlZW4gaW5pdGlhbGl6ZWQAAAAADV9fY29uc3RydWN0b3IAAAAAAAAIAAAAAAAAAAVvd25lcgAAAAAAABMAAAAAAAAAB21hbmFnZXIAAAAAEwAAAAAAAAALYWRtaW5fc2NvcGUAAAAABAAAAAAAAAAHZW1pdHRlcgAAAAATAAAAAAAAAAxib290c3RyYXBwZXIAAAATAAAAAAAAAA5iYWNrc3RvcF90b2tlbgAAAAAAEwAAAAAAAAAJYmFja3N0b3BzAAAAAAAD6gAAABMAAAAAAAAABXBvb2xzAAAAAAAD6gAAABMAAAAA",
        "AAAAAAAAAAlHZXQgb3duZXIAAAAAAAAFb3duZXIAAAAAAAAAAAAAAQAAABM=",
        "AAAAAAAAAAtHZXQgbWFuYWdlcgAAAAAHbWFuYWdlcgAAAAAAAAAAAQAAB9AAAAAHTWFuYWdlcgA=",
        "AAAAAAAAABhHZXQgdGhlIGVtaXR0ZXIgY29udHJhY3QAAAAHZW1pdHRlcgAAAAAAAAAAAQAAABM=",
        "AAAAAAAAACZHZXQgdGhlIGJhY2tzdG9wIGJvb3RzdHJhcHBlciBjb250cmFjdAAAAAAAFWJhY2tzdG9wX2Jvb3RzdHJhcHBlcgAAAAAAAAAAAAABAAAAEw==",
        "AAAAAAAAADFHZXQgdGhlIGJhY2tzdG9wcyB0aGlzIGNvbnRyYWN0IGNhbiBpbnRlcmFjdCB3aXRoAAAAAAAACWJhY2tzdG9wcwAAAAAAAAAAAAABAAAD6gAAABM=",
        "AAAAAAAAAC1HZXQgdGhlIHBvb2xzIHRoaXMgY29udHJhY3QgY2FuIGludGVyYWN0IHdpdGgAAAAAAAAFcG9vbHMAAAAAAAAAAAAAAQAAA+oAAAAT",
        "AAAAAAAAAOYoT25seSBPd25lcikgVHJhbnNmZXIgdG9rZW5zIGZyb20gdGhlIG1hbmFnZXIgY29udHJhY3QgdG8gYW5vdGhlciBhZGRyZXNzCgojIyMgQXJndW1lbnRzCiogYHRva2VuYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSB0b2tlbiB0byB0cmFuc2ZlcgoqIGB0b2AgLSBUaGUgYWRkcmVzcyB0byB0cmFuc2ZlciB0aGUgdG9rZW5zIHRvCiogYGFtb3VudGAgLSBUaGUgYW1vdW50IG9mIHRva2VucyB0byB0cmFuc2ZlcgAAAAAADnRyYW5zZmVyX3Rva2VuAAAAAAADAAAAAAAAAAV0b2tlbgAAAAAAABMAAAAAAAAAAnRvAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
        "AAAAAAAAAKcoT25seSBPd25lcikgU2V0IHRoZSBtYW5hZ2VyIGZvciB0aGUgY29udHJhY3QKCiMjIyBBcmd1bWVudHMKKiBgbWFuYWdlcmAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgbWFuYWdlcgoqIGBzY29wZWAgLSBUaGUgc2NvcGUgb2YgdGhlIG1hbmFnZXIuIDAgPSBOb25lLCAxID0gTG93LCAyID0gSGlnaAAAAAALc2V0X21hbmFnZXIAAAAAAgAAAAAAAAAHbWFuYWdlcgAAAAATAAAAAAAAAAVzY29wZQAAAAAAAAQAAAAA",
        "AAAAAAAAAIcoT25seSBPd25lcikgU2V0IHRoZSBiYWNrc3RvcCBib290c3RyYXBwZXIgY29udHJhY3QKCiMjIyBBcmd1bWVudHMKKiBgYm9vdHN0cmFwcGVyYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBib290c3RyYXBwZXIgY29udHJhY3QAAAAAGXNldF9iYWNrc3RvcF9ib290c3RyYXBwZXIAAAAAAAABAAAAAAAAAAxib290c3RyYXBwZXIAAAATAAAAAA==",
        "AAAAAAAAAIgoT25seSBPd25lcikgU2V0IHRoZSBsaXN0IG9mIHZhbGlkIGJhY2tzdG9wcwoKIyMjIEFyZ3VtZW50cwoqIGBiYWNrc3RvcHNgIC0gVGhlIGFkZHJlc3NlcyBvZiB0aGUgYmFja3N0b3BzIHRoZSBtYW5hZ2VyIGNhbiBpbnRlcmFjdCB3aXRoAAAADXNldF9iYWNrc3RvcHMAAAAAAAABAAAAAAAAAAliYWNrc3RvcHMAAAAAAAPqAAAAEwAAAAA=",
        "AAAAAAAAAIAoT25seSBPd25lcikgU2V0IHRoZSBsaXN0IG9mIHZhbGlkIHBvb2xzCgojIyMgQXJndW1lbnRzCiogYHBvb2xzYCAtIFRoZSBhZGRyZXNzZXMgb2YgdGhlIGJhY2tzdG9wcyB0aGUgbWFuYWdlciBjYW4gaW50ZXJhY3Qgd2l0aAAAAAlzZXRfcG9vbHMAAAAAAAABAAAAAAAAAAVwb29scwAAAAAAA+oAAAATAAAAAA==",
        "AAAAAAAAAN8oTWFuYWdlciwgTG93KSBUcmFuc2ZlciB0b2tlbnMgZnJvbSB0aGUgY29udHJhY3QgYmFjayB0byB0aGUgb3duZXIKCiMjIyBBcmd1bWVudHMKKiBgdG9rZW5gIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHRva2VuIHRvIHRyYW5zZmVyCiogYHRvYCAtIFRoZSBhZGRyZXNzIHRvIHRyYW5zZmVyIHRoZSB0b2tlbnMgdG8KKiBgYW1vdW50YCAtIFRoZSBhbW91bnQgb2YgdG9rZW5zIHRvIHRyYW5zZmVyAAAAAAxyZWZ1bmRfdG9rZW4AAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAFdG9rZW4AAAAAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
        "AAAAAAAAAVgoTWFuYWdlciwgTG93KSBDbGFpbSBiYWNrc3RvcCBkZXBvc2l0IGVtaXNzaW9ucyBmcm9tIGEgbGlzdCBvZiBwb29scyBmb3IgdGhlIGNvbnRyYWN0CgpSZXR1cm5zIHRoZSBhbW91bnQgb2YgQkxORCBlbWlzc2lvbnMgY2xhaW1lZAoKIyMjIEFyZ3VtZW50cwoqIGBmcm9tYCAtIFRoZSBjYWxsZXIgb2YgdGhlIGZ1bmN0aW9uCiogYGJhY2tzdG9wYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBjb250cmFjdAoqIGBwb29sX2FkZHJlc3NgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHBvb2wgdG8gY2xhaW0gZnJvbQoKIyMjIEVycm9ycwpJZiBhbiBpbnZhbGlkIHBvb2wgYWRkcmVzcyBpcyBpbmNsdWRlZAAAAAdiX2NsYWltAAAAAAMAAAAAAAAABGZyb20AAAATAAAAAAAAAAhiYWNrc3RvcAAAABMAAAAAAAAADHBvb2xfYWRkcmVzcwAAABMAAAABAAAACw==",
        "AAAAAAAAAVwoTWFuZ2VyLCBNZWRpdW0pIERlcG9zaXQgImFtb3VudCIgYmFja3N0b3AgdG9rZW5zIGZyb20gdGhlIGNvbnRyYWN0IGludG8gdGhlIGJhY2tzdG9wIGZvciAicG9vbF9hZGRyZXNzIgoKUmV0dXJucyB0aGUgbnVtYmVyIG9mIGJhY2tzdG9wIHBvb2wgc2hhcmVzIG1pbnRlZAoKIyMjIEFyZ3VtZW50cwoqIGBmcm9tYCAtIFRoZSBjYWxsZXIgb2YgdGhlIGZ1bmN0aW9uCiogYGJhY2tzdG9wYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBjb250cmFjdAoqIGBwb29sX2FkZHJlc3NgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHBvb2wKKiBgYW1vdW50YCAtIFRoZSBhbW91bnQgb2YgdG9rZW5zIHRvIGRlcG9zaXQAAAAJYl9kZXBvc2l0AAAAAAAABAAAAAAAAAAEZnJvbQAAABMAAAAAAAAACGJhY2tzdG9wAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAAAs=",
        "AAAAAAAAAWYoTWFuYWdlciwgTWVkaXVtKSBRdWV1ZSBkZXBvc2l0ZWQgcG9vbCBzaGFyZXMgZnJvbSB0aGUgY29udHJhY3QgZm9yIHdpdGhkcmF3IGZyb20gYSBiYWNrc3RvcCBvZiBhIHBvb2wKClJldHVybnMgdGhlIGNyZWF0ZWQgcXVldWUgZm9yIHdpdGhkcmF3YWwKCiMjIyBBcmd1bWVudHMKKiBgZnJvbWAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYmFja3N0b3AgY29udHJhY3QKKiBgYmFja3N0b3BgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIGNvbnRyYWN0CiogYHBvb2xfYWRkcmVzc2AgLSBUaGUgYWRkcmVzcyBvZiB0aGUgcG9vbAoqIGBhbW91bnRgIC0gVGhlIGFtb3VudCBvZiBzaGFyZXMgdG8gcXVldWUgZm9yIHdpdGhkcmF3AAAAAAASYl9xdWV1ZV93aXRoZHJhd2FsAAAAAAAEAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAIYmFja3N0b3AAAAATAAAAAAAAAAxwb29sX2FkZHJlc3MAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
        "AAAAAAAAAS4oTWFuYWdlciwgTWVkaXVtKSBEZXF1ZXVlIGEgY3VycmVudGx5IHF1ZXVlZCBwb29sIHNoYXJlIHdpdGhkcmF3IGZvciB0aGUgY29udHJhY3QgZnJvbSB0aGUgYmFja3N0b3Agb2YgYSBwb29sCgojIyMgQXJndW1lbnRzCiogYGZyb21gIC0gVGhlIGNhbGxlciBvZiB0aGUgZnVuY3Rpb24KKiBgYmFja3N0b3BgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIGNvbnRyYWN0CiogYHBvb2xfYWRkcmVzc2AgLSBUaGUgYWRkcmVzcyBvZiB0aGUgcG9vbAoqIGBhbW91bnRgIC0gVGhlIGFtb3VudCBvZiBzaGFyZXMgdG8gZGVxdWV1ZQAAAAAAFGJfZGVxdWV1ZV93aXRoZHJhd2FsAAAABAAAAAAAAAAEZnJvbQAAABMAAAAAAAAACGJhY2tzdG9wAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAA==",
        "AAAAAAAAAUQoTWFuYWdlciwgSGlnaCkgV2l0aGRyYXcgc2hhcmVzIGZyb20gdGhlIGNvbnRyYWN0J3Mgd2l0aGRyYXcgcXVldWUgZm9yIGEgYmFja3N0b3Agb2YgYSBwb29sCgpSZXR1cm5zIHRoZSBhbW91bnQgb2YgdG9rZW5zIHJldHVybmVkCgojIyMgQXJndW1lbnRzCiogYGZyb21gIC0gVGhlIGNhbGxlciBvZiB0aGUgZnVuY3Rpb24KKiBgYmFja3N0b3BgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIGNvbnRyYWN0CiogYHBvb2xfYWRkcmVzc2AgLSBUaGUgYWRkcmVzcyBvZiB0aGUgcG9vbAoqIGBhbW91bnRgIC0gVGhlIGFtb3VudCBvZiBzaGFyZXMgdG8gd2l0aGRyYXcAAAAKYl93aXRoZHJhdwAAAAAABAAAAAAAAAAEZnJvbQAAABMAAAAAAAAACGJhY2tzdG9wAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAAAs=",
        "AAAAAAAAAQcoTWFuYWdlciwgSGlnaCkgSm9pbiB0aGUgQkxORC1VU0RDIExQLgoKIyMjIEFyZ3VtZW50cwoqIGBmcm9tYCAtIFRoZSBjYWxsZXIgb2YgdGhlIGZ1bmN0aW9uCiogYGJhY2tzdG9wX3Rva2VuYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCB0b2tlbgoqIGBwb29sX2Ftb3VudF9vdXRgIC0gVGhlIGFtb3VudCBvZiBwb29sIHNoYXJlcyB0byBtaW50CiogYG1heF9hbW91bnRzX2luYCAtIFRoZSBtYXhpbXVtIGFtb3VudCBvZiB0b2tlbnMgdG8gZGVwb3NpdAAAAAALY19qb2luX3Bvb2wAAAAAAwAAAAAAAAAEZnJvbQAAABMAAAAAAAAAD3Bvb2xfYW1vdW50X291dAAAAAALAAAAAAAAAA5tYXhfYW1vdW50c19pbgAAAAAD6gAAAAsAAAAA",
        "AAAAAAAAAN4oTWFuYWdlciwgSGlnaCkgRXhpdCBhIGJhY2tzdG9wIHRva2VuJ3MgbGlxdWlkaXR5IHBvb2wuCgojIyMgQXJndW1lbnRzCiogYGZyb21gIC0gVGhlIGNhbGxlciBvZiB0aGUgZnVuY3Rpb24KKiBgYnVybl9hbW91bnRgIC0gVGhlIGFtb3VudCBvZiBwb29sIHNoYXJlcyB0byBidXJuCiogYG1pbl9hbW91bnRzX291dGAgLSBUaGUgbWluaW11bSBhbW91bnQgb2YgdG9rZW5zIHRvIHJlY2VpdmUAAAAAAAtjX2V4aXRfcG9vbAAAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAALYnVybl9hbW91bnQAAAAACwAAAAAAAAAPbWluX2Ftb3VudHNfb3V0AAAAA+oAAAALAAAAAA==",
        "AAAAAAAAAOEoTWFuYWdlciwgTG93KSBDbGFpbXMgdGhlIHByb2NlZWRzIG9mIGEgYmFja3N0b3AgYm9vdHN0cmFwcGluZwoKIyMjIEFyZ3VtZW50cwoqIGBmcm9tYCAtIFRoZSBjYWxsZXIgb2YgdGhlIGZ1bmN0aW9uCiogYGJvb3RzdHJhcF9pZGAgLSBUaGUgaWQgb2YgdGhlIGJvb3RzdHJhcHBlcgoqIGBiYWNrc3RvcGAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYmFja3N0b3AgdGhlIGJvb3RzdHJhcCBpcyBmb3IAAAAAAAASYmJfY2xhaW1fYm9vdHN0cmFwAAAAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAMYm9vdHN0cmFwX2lkAAAABAAAAAAAAAAIYmFja3N0b3AAAAATAAAAAA==",
        "AAAAAAAAAJwoTWFuYWdlciwgTG93KSBSZWZ1bmRzIGEgY2FuY2VsbGVkIGJhY2tzdG9wIGJvb3RzdHJhcHBpbmcKCiMjIyBBcmd1bWVudHMKKiBgZnJvbWAgLSBUaGUgY2FsbGVyIG9mIHRoZSBmdW5jdGlvbgoqIGBib290c3RyYXBfaWRgIC0gVGhlIGlkIG9mIHRoZSBib290c3RyYXBwZXIAAAATYmJfcmVmdW5kX2Jvb3RzdHJhcAAAAAACAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAMYm9vdHN0cmFwX2lkAAAABAAAAAA=",
        "AAAAAAAAAX0oTWFuYWdlciwgSGlnaCkgQ3JlYXRlcyBhIEJhY2tzdG9wIEJvb3RzdHJhcHBpbmcgd2l0aCBCTE5ECgojIyMgQXJndW1lbnRzCiogYGZyb21gIC0gVGhlIGNhbGxlciBvZiB0aGUgZnVuY3Rpb24KKiBgYm9vdHN0cmFwX3Rva2VuYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBib290c3RyYXAgdG9rZW4KKiBgYm9vdHN0cmFwX2Ftb3VudGAgLSBUaGUgYW1vdW50IG9mIHRva2VucyB0byBib290c3RyYXAKKiBgcGFpcl9taW5gIC0gVGhlIG1pbmltdW0gYW1vdW50IG9mIHBvb2wgc2hhcmVzIHRvIG1pbnQKKiBgZHVyYXRpb25gIC0gVGhlIGR1cmF0aW9uIG9mIHRoZSBib290c3RyYXBwaW5nIHBlcmlvZAoqIGBwb29sX2FkZHJlc3NgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIHBvb2wAAAAAAAASYmJfc3RhcnRfYm9vdHN0cmFwAAAAAAAGAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAVYm9vdHN0cmFwX3Rva2VuX2luZGV4AAAAAAAABAAAAAAAAAAQYm9vdHN0cmFwX2Ftb3VudAAAAAsAAAAAAAAACHBhaXJfbWluAAAACwAAAAAAAAAIZHVyYXRpb24AAAAEAAAAAAAAAAxwb29sX2FkZHJlc3MAAAATAAAAAA==",
        "AAAABAAAACFUaGUgZXJyb3IgY29kZXMgZm9yIHRoZSBjb250cmFjdC4AAAAAAAAAAAAAFUJhY2tzdG9wTWFuYWdlckVycm9ycwAAAAAAAAsAAAAAAAAADUludGVybmFsRXJyb3IAAAAAAAABAAAAAAAAABdBbHJlYWR5SW5pdGlhbGl6ZWRFcnJvcgAAAAADAAAAAAAAABFVbmF1dGhvcml6ZWRFcnJvcgAAAAAAAAQAAAAAAAAAE05lZ2F0aXZlQW1vdW50RXJyb3IAAAAACAAAAAAAAAAOQWxsb3dhbmNlRXJyb3IAAAAAAAkAAAAAAAAADEJhbGFuY2VFcnJvcgAAAAoAAAAAAAAADU92ZXJmbG93RXJyb3IAAAAAAAAMAAAAAAAAABNDb250cmFjdExpc3RPdmVyTWF4AAAAAGQAAAAAAAAAFkludmFsaWRDb250cmFjdEFkZHJlc3MAAAAAAGUAAAAAAAAADEludmFsaWRTY29wZQAAAGYAAAAAAAAAEUludmFsaWRUb2tlbkluZGV4AAAAAAAAZw==",
        "AAAAAQAAAAAAAAAAAAAAB01hbmFnZXIAAAAAAgAAABpUaGUgYWRkcmVzcyBvZiB0aGUgbWFuYWdlcgAAAAAAAmlkAAAAAAATAAAAN1RoZSBzY29wZSBvZiB0aGUgbWFuYWdlcgowIC0+IExvdwoxIC0+IE1lZGl1bQoyIC0+IEhpZ2gAAAAABXNjb3BlAAAAAAAABA==" ]),
      options
    )
  }
  public readonly fromJSON = {
    owner: this.txFromJSON<string>,
        manager: this.txFromJSON<Manager>,
        emitter: this.txFromJSON<string>,
        backstop_bootstrapper: this.txFromJSON<string>,
        backstops: this.txFromJSON<Array<string>>,
        pools: this.txFromJSON<Array<string>>,
        transfer_token: this.txFromJSON<null>,
        set_manager: this.txFromJSON<null>,
        set_backstop_bootstrapper: this.txFromJSON<null>,
        set_backstops: this.txFromJSON<null>,
        set_pools: this.txFromJSON<null>,
        refund_token: this.txFromJSON<null>,
        b_claim: this.txFromJSON<i128>,
        b_deposit: this.txFromJSON<i128>,
        b_queue_withdrawal: this.txFromJSON<null>,
        b_dequeue_withdrawal: this.txFromJSON<null>,
        b_withdraw: this.txFromJSON<i128>,
        c_join_pool: this.txFromJSON<null>,
        c_exit_pool: this.txFromJSON<null>,
        bb_claim_bootstrap: this.txFromJSON<null>,
        bb_refund_bootstrap: this.txFromJSON<null>,
        bb_start_bootstrap: this.txFromJSON<null>
  }
}