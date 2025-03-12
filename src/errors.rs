use soroban_sdk::contracterror;

/// The error codes for the contract.
#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BackstopManagerErrors {
    // Default errors to align with built-in contract
    InternalError = 1,
    AlreadyInitializedError = 3,

    UnauthorizedError = 4,

    NegativeAmountError = 8,
    AllowanceError = 9,
    BalanceError = 10,
    OverflowError = 12,

    ContractListOverMax = 100,
    InvalidContractAddress = 101,
    InvalidScope = 102,
    InvalidTokenIndex = 103,
}
