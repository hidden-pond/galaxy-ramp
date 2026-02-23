use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum PoolError {
    AlreadyInitialized = 201,

    OperationIdAlreadyConsumed = 2300,
    SwapNotPerformed = 2301,
    UnauthorizedOperator = 2302,
    OperationIdNotConsumed = 2303,
    FeeExceedSwapAmount = 2304,
}
