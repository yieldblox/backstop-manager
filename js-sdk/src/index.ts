import { Contract } from "@stellar/stellar-sdk";
import { Spec } from "@stellar/stellar-sdk/contract";
import { Buffer } from "buffer";

if (typeof window !== "undefined") {
  //@ts-ignore Buffer exists
  window.Buffer = window.Buffer || Buffer;
}

/**
 * The error codes for the contract.
 */
export const Errors = {
  1: { message: "InternalError" },

  3: { message: "AlreadyInitializedError" },

  4: { message: "UnauthorizedError" },

  8: { message: "NegativeAmountError" },

  9: { message: "AllowanceError" },

  10: { message: "BalanceError" },

  12: { message: "OverflowError" },

  100: { message: "InvalidUnlockTime" },

  101: { message: "InvalidContractAddress" },

  102: { message: "InvalidTokenIndex" },
};

export const BLEND_LOCKUP_SPEC = new Spec([
  "AAAAAAAAAUNJbml0aWFsaXplIHRoZSBjb250cmFjdAoKIyMjIEFyZ3VtZW50cwoqIG93bmVyIC0gVGhlIG93bmVyIG9mIHRoZSBjb250cmFjdAoqIGVtaXR0ZXIgLSBUaGUgYWRkcmVzcyBvZiB0aGUgZW1pdHRlciBjb250cmFjdAoqIGJvb3RzdHJhcHBlciAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBib290c3RyYXBwZXIgY29udHJhY3QKKiB1bmxvY2sgLSBUaGUgdW5sb2NrIHRpbWUgKGluIHNlY29uZHMgc2luY2UgZXBvY2gpCgojIyMgRXJyb3JzCiogQWxyZWFkeUluaXRpYWxpemVkRXJyb3IgLSBUaGUgY29udHJhY3QgaGFzIGFscmVhZHkgYmVlbiBpbml0aWFsaXplZAAAAAAKaW5pdGlhbGl6ZQAAAAAABAAAAAAAAAAFb3duZXIAAAAAAAATAAAAAAAAAAdlbWl0dGVyAAAAABMAAAAAAAAADGJvb3RzdHJhcHBlcgAAABMAAAAAAAAABnVubG9jawAAAAAABgAAAAA=",
  "AAAAAAAAAAlHZXQgb3duZXIAAAAAAAAFb3duZXIAAAAAAAAAAAAAAQAAABM=",
  "AAAAAAAAAB1HZXQgdW5sb2NrIHRpbWUgb2YgdGhlIGxvY2t1cAAAAAAAAAZ1bmxvY2sAAAAAAAAAAAABAAAABg==",
  "AAAAAAAAABhHZXQgdGhlIGVtaXR0ZXIgY29udHJhY3QAAAAHZW1pdHRlcgAAAAAAAAAAAQAAABM=",
  "AAAAAAAAAEFHZXQgdGhlIGJhY2tzdG9wIGNvbnRyYWN0cyB0aGF0IGhhdmUgYmVlbiByZWNvcmRlZCBieSB0aGUgZW1pdHRlcgAAAAAAAAliYWNrc3RvcHMAAAAAAAAAAAAAAQAAA+oAAAAT",
  "AAAAAAAAAEdHZXQgdGhlIGJhY2tzdG9wIHRva2VuIGNvbnRyYWN0cyB0aGF0IGhhdmUgYmVlbiByZWNvcmRlZCBieSB0aGUgZW1pdHRlcgAAAAAPYmFja3N0b3BfdG9rZW5zAAAAAAAAAAABAAAD6gAAABM=",
  "AAAAAAAAAE4oT25seSBPd25lcikgVXBkYXRlIHRoZSBiYWNrc3RvcCBjb250cmFjdCBhbmQgdG9rZW4gZnJvbSB0aGUgZW1pdHRlciBjb250cmFjdC4AAAAAAA91cGRhdGVfYmFja3N0b3AAAAAAAAAAAAA=",
  "AAAAAAAAALgoT25seSBPd25lcikgQ2xhaW0gYXNzZXRzIGZyb20gdGhlIGxvY2t1cAoKIyMjIEFyZ3VtZW50cwoqIGBhc3NldHNgIC0gVGhlIFZlYyBvZiBhZGRyZXNzZXMgb2YgdGhlIGFzc2V0cyB0byBjbGFpbQoKIyMjIEVycm9ycwoqIEludmFsaWRVbmxvY2tUaW1lIC0gVGhlIHVubG9jayB0aW1lIGhhcyBub3QgYmVlbiByZWFjaGVkAAAABWNsYWltAAAAAAAAAQAAAAAAAAAGYXNzZXRzAAAAAAPqAAAAEwAAAAA=",
  "AAAAAAAAAWcoT25seSBPd25lcikgRGVwb3NpdCAiYW1vdW50IiBiYWNrc3RvcCB0b2tlbnMgZnJvbSB0aGUgbG9ja3VwIGludG8gdGhlIGJhY2tzdG9wIGZvciAicG9vbF9hZGRyZXNzIgoKUmV0dXJucyB0aGUgbnVtYmVyIG9mIGJhY2tzdG9wIHBvb2wgc2hhcmVzIG1pbnRlZAoKIyMjIEFyZ3VtZW50cwoqIGBiYWNrc3RvcGAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYmFja3N0b3AgY29udHJhY3QKKiBgYmFja3N0b3BfdG9rZW5gIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIHRva2VuCiogYHBvb2xfYWRkcmVzc2AgLSBUaGUgYWRkcmVzcyBvZiB0aGUgcG9vbAoqIGBhbW91bnRgIC0gVGhlIGFtb3VudCBvZiB0b2tlbnMgdG8gZGVwb3NpdAAAAAAJYl9kZXBvc2l0AAAAAAAABAAAAAAAAAAIYmFja3N0b3AAAAATAAAAAAAAAA5iYWNrc3RvcF90b2tlbgAAAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAAAs=",
  "AAAAAAAAAS8oT25seSBPd25lcikgUXVldWUgZGVwb3NpdGVkIHBvb2wgc2hhcmVzIGZyb20gdGhlIGxvY2t1cCBmb3Igd2l0aGRyYXcgZnJvbSBhIGJhY2tzdG9wIG9mIGEgcG9vbAoKUmV0dXJucyB0aGUgY3JlYXRlZCBxdWV1ZSBmb3Igd2l0aGRyYXdhbAoKIyMjIEFyZ3VtZW50cwoqIGBiYWNrc3RvcGAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYmFja3N0b3AgY29udHJhY3QKKiBgcG9vbF9hZGRyZXNzYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBwb29sCiogYGFtb3VudGAgLSBUaGUgYW1vdW50IG9mIHNoYXJlcyB0byBxdWV1ZSBmb3Igd2l0aGRyYXcAAAAAEmJfcXVldWVfd2l0aGRyYXdhbAAAAAAAAwAAAAAAAAAIYmFja3N0b3AAAAATAAAAAAAAAAxwb29sX2FkZHJlc3MAAAATAAAAAAAAAAZhbW91bnQAAAAAAAsAAAAA",
  "AAAAAAAAAQEoT25seSBPd25lcikgRGVxdWV1ZSBhIGN1cnJlbnRseSBxdWV1ZWQgcG9vbCBzaGFyZSB3aXRoZHJhdyBmb3IgdGhlIGxvY2t1cCBmcm9tIHRoZSBiYWNrc3RvcCBvZiBhIHBvb2wKCiMjIyBBcmd1bWVudHMKKiBgYmFja3N0b3BgIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIGNvbnRyYWN0CiogYHBvb2xfYWRkcmVzc2AgLSBUaGUgYWRkcmVzcyBvZiB0aGUgcG9vbAoqIGBhbW91bnRgIC0gVGhlIGFtb3VudCBvZiBzaGFyZXMgdG8gZGVxdWV1ZQAAAAAAABRiX2RlcXVldWVfd2l0aGRyYXdhbAAAAAMAAAAAAAAACGJhY2tzdG9wAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAA==",
  "AAAAAAAAARkoT25seSBPd25lcikgV2l0aGRyYXcgc2hhcmVzIGZyb20gdGhlIGxvY2t1cCdzIHdpdGhkcmF3IHF1ZXVlIGZvciBhIGJhY2tzdG9wIG9mIGEgcG9vbAoKUmV0dXJucyB0aGUgYW1vdW50IG9mIHRva2VucyByZXR1cm5lZAoKIyMjIEFyZ3VtZW50cwoqIGBiYWNrc3RvcGAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYmFja3N0b3AgY29udHJhY3QKKiBgcG9vbF9hZGRyZXNzYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBwb29sCiogYGFtb3VudGAgLSBUaGUgYW1vdW50IG9mIHNoYXJlcyB0byB3aXRoZHJhdwAAAAAAAApiX3dpdGhkcmF3AAAAAAADAAAAAAAAAARmcm9tAAAAEwAAAAAAAAAMcG9vbF9hZGRyZXNzAAAAEwAAAAAAAAAGYW1vdW50AAAAAAALAAAAAQAAAAs=",
  "AAAAAAAAAUgoT25seSBPd25lcikgQ2xhaW0gYmFja3N0b3AgZGVwb3NpdCBlbWlzc2lvbnMgZnJvbSBhIGxpc3Qgb2YgcG9vbHMgZm9yIHRoZSBsb2NrdXAKClJldHVybnMgdGhlIGFtb3VudCBvZiBCTE5EIGVtaXNzaW9ucyBjbGFpbWVkCgojIyMgQXJndW1lbnRzCiogYGJhY2tzdG9wYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBiYWNrc3RvcCBjb250cmFjdAoqIGBwb29sX2FkZHJlc3Nlc2AgLSBUaGUgVmVjIG9mIGFkZHJlc3NlcyB0byBjbGFpbSBiYWNrc3RvcCBkZXBvc2l0IGVtaXNzaW9ucyBmcm9tCgojIyMgRXJyb3JzCklmIGFuIGludmFsaWQgcG9vbCBhZGRyZXNzIGlzIGluY2x1ZGVkAAAAB2JfY2xhaW0AAAAAAgAAAAAAAAAIYmFja3N0b3AAAAATAAAAAAAAAA5wb29sX2FkZHJlc3NlcwAAAAAD6gAAABMAAAABAAAACw==",
  "AAAAAAAAATAoT25seSBPd25lcikgSm9pbiBhIGJhY2tzdG9wIHRva2VuJ3MgbGlxdWlkaXR5IHBvb2wuIFJlcXVpcmVzIHRoYXQgdGhlIGJhY2tzdG9wIHRva2VuIGltcGxlbWVudHMgdGhlIENvbWV0IGludGVyZmFjZS4KCiMjIyBBcmd1bWVudHMKKiBgYmFja3N0b3BfdG9rZW5gIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIHRva2VuCiogYHBvb2xfYW1vdW50X291dGAgLSBUaGUgYW1vdW50IG9mIHBvb2wgc2hhcmVzIHRvIG1pbnQKKiBgbWF4X2Ftb3VudHNfaW5gIC0gVGhlIG1heGltdW0gYW1vdW50IG9mIHRva2VucyB0byBkZXBvc2l0AAAAC2Nfam9pbl9wb29sAAAAAAMAAAAAAAAADmJhY2tzdG9wX3Rva2VuAAAAAAATAAAAAAAAAA9wb29sX2Ftb3VudF9vdXQAAAAACwAAAAAAAAAObWF4X2Ftb3VudHNfaW4AAAAAA+oAAAALAAAAAA==",
  "AAAAAAAAAS0oT25seSBPd25lcikgRXhpdCBhIGJhY2tzdG9wIHRva2VuJ3MgbGlxdWlkaXR5IHBvb2wuIFJlcXVpcmVzIHRoYXQgdGhlIGJhY2tzdG9wIHRva2VuIGltcGxlbWVudHMgdGhlIENvbWV0IGludGVyZmFjZS4KCiMjIyBBcmd1bWVudHMKKiBgYmFja3N0b3BfdG9rZW5gIC0gVGhlIGFkZHJlc3Mgb2YgdGhlIGJhY2tzdG9wIHRva2VuCiogYGJ1cm5fYW1vdW50YCAtIFRoZSBhbW91bnQgb2YgcG9vbCBzaGFyZXMgdG8gYnVybgoqIGBtaW5fYW1vdW50c19vdXRgIC0gVGhlIG1pbmltdW0gYW1vdW50IG9mIHRva2VucyB0byByZWNlaXZlAAAAAAAAC2NfZXhpdF9wb29sAAAAAAMAAAAAAAAADmJhY2tzdG9wX3Rva2VuAAAAAAATAAAAAAAAAAtidXJuX2Ftb3VudAAAAAALAAAAAAAAAA9taW5fYW1vdW50c19vdXQAAAAD6gAAAAsAAAAA",
  "AAAAAAAAAVQoT25seSBPd25lcikgQ3JlYXRlcyBhIEJhY2tzdG9wIEJvb3RzdHJhcHBpbmcgd2l0aCBCTE5ECgojIyMgQXJndW1lbnRzCiogYGJvb3RzdHJhcF90b2tlbmAgLSBUaGUgYWRkcmVzcyBvZiB0aGUgYm9vdHN0cmFwIHRva2VuCiogYGJvb3RzdHJhcF9hbW91bnRgIC0gVGhlIGFtb3VudCBvZiB0b2tlbnMgdG8gYm9vdHN0cmFwCiogYHBhaXJfbWluYCAtIFRoZSBtaW5pbXVtIGFtb3VudCBvZiBwb29sIHNoYXJlcyB0byBtaW50CiogYGR1cmF0aW9uYCAtIFRoZSBkdXJhdGlvbiBvZiB0aGUgYm9vdHN0cmFwcGluZyBwZXJpb2QKKiBgcG9vbF9hZGRyZXNzYCAtIFRoZSBhZGRyZXNzIG9mIHRoZSBwb29sAAAAEmJiX3N0YXJ0X2Jvb3RzdHJhcAAAAAAABQAAAAAAAAAVYm9vdHN0cmFwX3Rva2VuX2luZGV4AAAAAAAABAAAAAAAAAAQYm9vdHN0cmFwX2Ftb3VudAAAAAsAAAAAAAAACHBhaXJfbWluAAAACwAAAAAAAAAIZHVyYXRpb24AAAAEAAAAAAAAAAxwb29sX2FkZHJlc3MAAAATAAAAAA==",
  "AAAAAAAAAOMoT25seSBPd25lcikgQ2xhaW1zIHRoZSBwcm9jZWVkcyBvZiBhIGJhY2tzdG9wIGJvb3RzdHJhcHBpbmcKCiMjIyBBcmd1bWVudHMKKiBgYm9vdHN0cmFwX2lkYCAtIFRoZSBpZCBvZiB0aGUgYm9vdHN0cmFwcGVyCiogYGJvb3RzdHJhcF90b2tlbl9pbmRleGAgLSBUaGUgaW5kZXggb2YgdGhlIHRva2VuIGJlaW5nIGJvb3RzdHJhcHBlZCAobXVzdCBtYXRjaCB3aXRoIGNsYWltZWQgYm9vdHN0cmFwKQAAAAASYmJfY2xhaW1fYm9vdHN0cmFwAAAAAAABAAAAAAAAAAxib290c3RyYXBfaWQAAAAEAAAAAA==",
  "AAAAAAAAAHQoT25seSBPd25lcikgUmVmdW5kcyBhIGNhbmNlbGxlZCBiYWNrc3RvcCBib290c3RyYXBwaW5nCgojIyMgQXJndW1lbnRzCiogYGJvb3RzdHJhcF9pZGAgLSBUaGUgaWQgb2YgdGhlIGJvb3RzdHJhcHBlcgAAABNiYl9yZWZ1bmRfYm9vdHN0cmFwAAAAAAEAAAAAAAAADGJvb3RzdHJhcF9pZAAAAAQAAAAA",
  "AAAABAAAACFUaGUgZXJyb3IgY29kZXMgZm9yIHRoZSBjb250cmFjdC4AAAAAAAAAAAAAEEJsZW5kTG9ja3VwRXJyb3IAAAAKAAAAAAAAAA1JbnRlcm5hbEVycm9yAAAAAAAAAQAAAAAAAAAXQWxyZWFkeUluaXRpYWxpemVkRXJyb3IAAAAAAwAAAAAAAAARVW5hdXRob3JpemVkRXJyb3IAAAAAAAAEAAAAAAAAABNOZWdhdGl2ZUFtb3VudEVycm9yAAAAAAgAAAAAAAAADkFsbG93YW5jZUVycm9yAAAAAAAJAAAAAAAAAAxCYWxhbmNlRXJyb3IAAAAKAAAAAAAAAA1PdmVyZmxvd0Vycm9yAAAAAAAADAAAAAAAAAARSW52YWxpZFVubG9ja1RpbWUAAAAAAABkAAAAAAAAABZJbnZhbGlkQ29udHJhY3RBZGRyZXNzAAAAAABlAAAAAAAAABFJbnZhbGlkVG9rZW5JbmRleAAAAAAAAGY=",
]);

export class BlendLockupContract extends Contract {
  constructor(contractId: string) {
    super(contractId);
  }

  static readonly parsers = {
    owner: (result: string): string => {
      return BLEND_LOCKUP_SPEC.funcResToNative("owner", result);
    },
    unlock: (result: string): bigint => {
      return BLEND_LOCKUP_SPEC.funcResToNative("unlock", result);
    },
    emitter: (result: string): string => {
      return BLEND_LOCKUP_SPEC.funcResToNative("emitter", result);
    },
    backstops: (result: string): string[] => {
      return BLEND_LOCKUP_SPEC.funcResToNative("backstops", result);
    },
    backstop_tokens: (result: string): string[] => {
      return BLEND_LOCKUP_SPEC.funcResToNative("backstop_tokens", result);
    },
    update_backstop: (): void => {},
    claim: (): void => {},
    b_deposit: (result: string): bigint => {
      return BLEND_LOCKUP_SPEC.funcResToNative("b_deposit", result);
    },
    b_queue_withdrawal: (): void => {},
    b_dequeue_withdrawal: (): void => {},
    b_withdraw: (result: string): bigint => {
      return BLEND_LOCKUP_SPEC.funcResToNative("b_withdraw", result);
    },
    b_claim: (result: string): bigint => {
      return BLEND_LOCKUP_SPEC.funcResToNative("b_claim", result);
    },
    c_join_pool: (): void => {},
    c_exit_pool: (): void => {},
  };

  /**
   * Construct a base64 XDR operation to invoke the "owner" function.
   *
   * `owner`
   *
   * Get the owner of the lockup.
   */
  owner(): string {
    return this.call("owner").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "unlock" function.
   *
   * `unlock`
   *
   * Get unlock time of the lockup
   */
  unlock(): string {
    return this.call("unlock").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "emitter" function.
   *
   * `emitter`
   *
   * Get the emitter of the lockup
   */
  emitter(): string {
    return this.call("emitter").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "backstops" function.
   *
   * `backstops`
   *
   * Get the backstop contracts that have been recorded by the emitter
   */
  backstops(): string {
    return this.call("backstops").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "backstop_tokens" function.
   *
   * `backstop_tokens`
   *
   * Get the backstop token contracts that have been recorded by the emitter
   */
  backstop_tokens(): string {
    return this.call("backstop_tokens").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "update_backstop" function.
   *
   * `update_backstop`
   *
   * (Only Owner) Update the backstop contract and token from the emitter contract.
   */
  update_backstop(): string {
    return this.call("update_backstop").toXDR("base64");
  }

  /**
   * Construct a base64 XDR operation to invoke the "claim" function.
   *
   * `claim`
   *
   * (Only Owner) Claim assets from the lockup
   *
   * @param assets - The Vec of addresses of the assets to claim
   */
  claim(assets: string[]): string {
    return this.call(
      "claim",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("claim", { assets })
    ).toXDR("base64");
  }

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
  b_deposit(
    backstop: string,
    backstop_token: string,
    pool_address: string,
    amount: bigint
  ): string {
    return this.call(
      "b_deposit",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("b_deposit", {
        backstop,
        backstop_token,
        pool_address,
        amount,
      })
    ).toXDR("base64");
  }

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
  b_queue_withdrawal(
    backstop: string,
    pool_address: string,
    amount: bigint
  ): string {
    return this.call(
      "b_queue_withdrawal",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("b_queue_withdrawal", {
        backstop,
        pool_address,
        amount,
      })
    ).toXDR("base64");
  }

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
  b_dequeue_withdrawal(
    backstop: string,
    pool_address: string,
    amount: bigint
  ): string {
    return this.call(
      "b_dequeue_withdrawal",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("b_dequeue_withdrawal", {
        backstop,
        pool_address,
        amount,
      })
    ).toXDR("base64");
  }

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
  b_withdraw(from: string, pool_address: string, amount: bigint): string {
    return this.call(
      "b_withdraw",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("b_withdraw", {
        from,
        pool_address,
        amount,
      })
    ).toXDR("base64");
  }

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
  b_claim(backstop: string, pool_addresses: string[]): string {
    return this.call(
      "b_claim",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("b_claim", {
        backstop,
        pool_addresses,
      })
    ).toXDR("base64");
  }

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
  c_join_pool(
    backstop_token: string,
    pool_amount_out: bigint,
    max_amounts_in: bigint[]
  ): string {
    return this.call(
      "c_join_pool",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("c_join_pool", {
        backstop_token,
        pool_amount_out,
        max_amounts_in,
      })
    ).toXDR("base64");
  }

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
  c_exit_pool(
    backstop_token: string,
    burn_amount: bigint,
    min_amounts_out: bigint[]
  ): string {
    return this.call(
      "c_exit_pool",
      ...BLEND_LOCKUP_SPEC.funcArgsToScVals("c_exit_pool", {
        backstop_token,
        burn_amount,
        min_amounts_out,
      })
    ).toXDR("base64");
  }
}
