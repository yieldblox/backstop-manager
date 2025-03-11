# backstop-manager

Contract that allows an owner to delegate management for a backstop deposit to another key. This is useful for DAO and/or similar governered owners where actions often require some voting period or delay to take place. The admin can take actions on behalf of the owner, like start bootstraps, queued deposits for withdraw, and deposit new LP tokens.

The owner can:
* Manage the manager key, including reducing the scope in which they can act
* Add/remove pools that the contract can move funds between
* Add/remove backstops that the contract can interact with
* Perform all manager actions

Manager actions are limited by scope. Their scope can be:
* Low - they can only perform basic actions
* Medium - they can perform some management actions
* High - they can perform all management actions

All manager actions only allow funds to be held by the owner. They act on behalf of the owner with the funds held by the backstop manager contract.

The admin can:
* Claim emissions from a backstop (scope req. `Low`)
* Deposit BLND-USDC LP tokens into a backstop (scope req. `Medium`)
* Queue for withdraw funds from backstop (scope req. `Medium`)
* Cancel queue funds for withdraw from backstop (scope req. `Medium`)
* Withdraw funds from backstop (scope req. `High`)
* Join and/or Exit BLND-USDC LP tokens from the comet pool (scope req. `High`)
* Start bootstraps with BLND or USDC held by the contract (scope req. `High`)
* Claim or refund bootstraps started by the contract (scope req. `Low`)
