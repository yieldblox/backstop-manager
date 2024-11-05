# blend-lockup

Lockup contract for BLND tokens dropped by the Backstop, that are able to safely interact with the Backstop.

Below are examples in JS with how to interact with the Blend Lockup contract using the provided SDK. These examples explain only how to create the `invokeHostOperation` needed to create a Soroban transaction. For more information about simulating and submitting Soroban transactions in JS, please see: https://developers.stellar.org/docs/build/guides/transactions

## Lockup

The lockup contract locks all tokens owned by the contract until the `unlock` date has been passed.

### Unlock

The unlock time can be fetched with the `unlock` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const operation = lockup.unlock();
// simulate operation
const result = BlendLockupContract.parsers.unlock("return result as base64 xdr string");
```

### Claim

After the unlock time has passed, tokens can be claimed with the `claim` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const operation = lockup.claim(["token_address_to_claim"]);
// simulate and/or submit operation
```

## Backstop

The lockup contract can use its tokens to interact with the backstop.

### Deposit

Backstop tokens can be deposited into the contract with the `d_deposit` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const operation = lockup.deposit(
  "backstop_address",
  "backstop_token_address",
  "pool_address",
  BigInt(123)
);
// simulate and/or submit operation
```

### Withdraw

Backstop tokens can be withdrawn via the `b_queue_withdrawal` function and the `b_withdraw` function.

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const queue_operation = lockup.b_queue_withdrawal(
  "backstop_address",
  "pool_address",
  BigInt(123)
);
// simulate and/or submit operation

const withdraw_operation = lockup.b_withdraw(
  "backstop_address",
  "pool_address",
  BigInt(123)
);
// simulate and/or submit operation
```

### Claim

Backstop deposits earn emissions from the Blend protocol. To claim them, invoke the `b_claim` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const operation = lockup.claim(
  "backstop_address",
  ["pool_address"],
);
// simulate and/or submit operation
```

### Backstop Upgrade

The Blend protocol can have a backstop upgrade occur. To allow the Blend Lockup contract to interact with the new backstop and backstop_token, invoke the `update_backstop` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const operation = lockup.update_backstop();
// simulate and/or submit operation
```

## Backstop Token

The Blend lockup contract can both mint and burn backstop tokens. The backstop token is Comet BLND:USDC 80/20 AMM shares. For more information, please see: https://docs.blend.capital/users/backstopping.

### Join

To join the Comet AMM, invoke the `c_join_pool` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const to_mint_shares = BigInt(123);
const max_blnd_in = BigInt(123);
const max_usdc_in = BigInt(123);
const operation = lockup.c_join_pool("pool_address", to_mint_shares, [
  max_blnd_in,
  max_usdc_in,
]);
// simulate and/or submit operation
```

### Exit

To exit the Comet AMM, invoke the `c_exit_pool` function:

```js
import { BlendLockupContract } from "./js-sdk";

const lockup = new BlendLockupContract("lockup_address");
const to_burn_shares = BigInt(123);
const min_blnd_out = BigInt(123);
const min_usdc_out = BigInt(123);
const operation = lockup.c_exit_pool("pool_address", to_burn_shares, [
  min_blnd_out,
  min_usdc_out,
]);
// simulate and/or submit operation
```

