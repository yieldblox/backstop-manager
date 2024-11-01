import { Contract } from "@stellar/stellar-sdk";
import { Spec } from "@stellar/stellar-sdk/contract";
/**
 * The error codes for the contract.
 */
export declare const Errors: {
    1: {
        message: string;
    };
    3: {
        message: string;
    };
    4: {
        message: string;
    };
    8: {
        message: string;
    };
    9: {
        message: string;
    };
    10: {
        message: string;
    };
    12: {
        message: string;
    };
    100: {
        message: string;
    };
    101: {
        message: string;
    };
    102: {
        message: string;
    };
};
export declare const BLEND_LOCKUP_SPEC: Spec;
export declare class BlendLockupContract extends Contract {
    constructor(contractId: string);
    static readonly parsers: {
        owner: (result: string) => string;
        unlock: (result: string) => bigint;
        emitter: (result: string) => string;
        backstops: (result: string) => string[];
        backstop_tokens: (result: string) => string[];
        update_backstop: () => void;
        claim: () => void;
        b_deposit: (result: string) => bigint;
        b_queue_withdrawal: () => void;
        b_dequeue_withdrawal: () => void;
        b_withdraw: (result: string) => bigint;
        b_claim: (result: string) => bigint;
        c_join_pool: () => void;
        c_exit_pool: () => void;
    };
    /**
     * Construct a base64 XDR operation to invoke the "owner" function.
     *
     * `owner`
     *
     * Get the owner of the lockup.
     */
    owner(): string;
    /**
     * Construct a base64 XDR operation to invoke the "unlock" function.
     *
     * `unlock`
     *
     * Get unlock time of the lockup
     */
    unlock(): string;
    /**
     * Construct a base64 XDR operation to invoke the "emitter" function.
     *
     * `emitter`
     *
     * Get the emitter of the lockup
     */
    emitter(): string;
    /**
     * Construct a base64 XDR operation to invoke the "backstops" function.
     *
     * `backstops`
     *
     * Get the backstop contracts that have been recorded by the emitter
     */
    backstops(): string;
    /**
     * Construct a base64 XDR operation to invoke the "backstop_tokens" function.
     *
     * `backstop_tokens`
     *
     * Get the backstop token contracts that have been recorded by the emitter
     */
    backstop_tokens(): string;
    /**
     * Construct a base64 XDR operation to invoke the "update_backstop" function.
     *
     * `update_backstop`
     *
     * (Only Owner) Update the backstop contract and token from the emitter contract.
     */
    update_backstop(): string;
    /**
     * Construct a base64 XDR operation to invoke the "claim" function.
     *
     * `claim`
     *
     * (Only Owner) Claim assets from the lockup
     *
     * @param assets - The Vec of addresses of the assets to claim
     */
    claim(assets: string[]): string;
    /**
     * Construct a base64 XDR operation to invoke the "b_deposit" function.
     *
     * `b_deposit`
     *
     * (Only Owner) Deposit "amount" backstop tokens from the lockup into the backstop for "pool_address"
     *
     * Returns the number of backstop pool shares minted
     *
     * @param backstop - The address of the backstop contract
     * @param backstop_token - The address of the backstop token
     * @param pool_address - The address of the pool
     * @param amount - The amount of tokens to deposit
     */
    b_deposit(backstop: string, backstop_token: string, pool_address: string, amount: bigint): string;
    /**
     * Construct a base64 XDR operation to invoke the "b_queue_withdrawal" function.
     *
     * `b_queue_withdrawal`
     *
     * (Only Owner) Queue deposited pool shares from the lockup for withdraw from a backstop of a pool
     *
     * Returns the created queue for withdrawal
     *
     * @param backstop - The address of the backstop contract
     * @param pool_address - The address of the pool
     * @param amount - The amount of shares to queue for withdraw
     */
    b_queue_withdrawal(backstop: string, pool_address: string, amount: bigint): string;
    /**
     * Construct a base64 XDR operation to invoke the "b_dequeue_withdrawal" function.
     *
     * `b_dequeue_withdrawal`
     *
     * (Only Owner) Dequeue a currently queued pool share withdraw for the lockup from the backstop of a pool
     *
     * @param backstop - The address of the backstop contract
     * @param pool_address - The address of the pool
     * @param amount - The amount of shares to dequeue
     */
    b_dequeue_withdrawal(backstop: string, pool_address: string, amount: bigint): string;
    /**
     * Construct a base64 XDR operation to invoke the "b_withdraw" function.
     *
     * `b_withdraw`
     *
     * (Only Owner) Withdraw shares from the lockup's withdraw queue for a backstop of a pool
     *
     * Returns the amount of tokens returned
     *
     * @param from - The address of the backstop contract
     * @param pool_address - The address of the pool
     * @param amount - The amount of shares to withdraw
     */
    b_withdraw(from: string, pool_address: string, amount: bigint): string;
    /**
     * Construct a base64 XDR operation to invoke the "b_claim" function.
     *
     * `b_claim`
     *
     * (Only Owner) Claim backstop deposit emissions from a list of pools for the lockup
     *
     * Returns the amount of BLND emissions claimed
     *
     * @param backstop - The address of the backstop contract
     * @param pool_addresses - The array of addresses to claim backstop deposit emissions from
     * @throws If an invalid pool address is included
     */
    b_claim(backstop: string, pool_addresses: string[]): string;
    /**
     * Construct a base64 XDR operation to invoke the "c_join_pool" function.
     *
     * `c_join_pool`
     *
     * (Only Owner) Join a backstop token's liquidity pool. Requires that the backstop token implements the Comet interface.
     *
     * @param backstop_token - The address of the backstop token
     * @param pool_amount_out - The amount of pool shares to mint
     * @param max_amounts_in - The maximum amount of tokens to deposit
     */
    c_join_pool(backstop_token: string, pool_amount_out: bigint, max_amounts_in: bigint[]): string;
    /**
     * Construct a base64 XDR operation to invoke the "c_exit_pool" function.
     *
     * `c_exit_pool`
     *
     * (Only Owner) Exit a backstop token's liquidity pool. Requires that the backstop token implements the Comet interface.
     *
     * @param backstop_token - The address of the backstop token
     * @param burn_amount - The amount of pool shares to burn
     * @param min_amounts_out - The minimum amount of tokens to receive
     */
    c_exit_pool(backstop_token: string, burn_amount: bigint, min_amounts_out: bigint[]): string;
}
