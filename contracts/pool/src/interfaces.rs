use soroban_sdk::{Address, BytesN, Env, Vec};

pub trait PoolContractInterface {
    fn set_admin(e: Env, admin: Address);
    fn set_token_in(e: Env, admin: Address, token_in: Address);

    fn set_operator(e: Env, admin: Address, operator: Address);

    fn set_swap_router(e: Env, admin: Address, swap_router: Address);

    fn add_request(
        e: Env,
        operator: Address,
        proxy_wallet: Address,
        token_out: Address,
        tx_id: BytesN<32>,
        op_id: u128,
        destination: Address,
        amount_in: i128,
    );

    fn cancel_request(
        e: Env,
        operator: Address,
        proxy_wallet: Address,
        op_id: u128,
        destination: Address,
    );

    fn terminate_request(
        e: Env,
        operator: Address,
        op_id: u128,
        destination: Address,
    );

    fn withdraw_token(
        e: Env,
        operator: Address,
        destination: Address,
        token: Address,
        amount: i128,
    );

    fn swap_chained_via_router(
        e: Env,
        operator: Address,
        destination: Address,
        op_id: u128,
        swaps_chain: Vec<(Vec<Address>, BytesN<32>, Address)>,
        out_min: i128,
    ) -> i128; // public getters
    fn get_token_in(e: Env) -> Address; // getters
                                        // get_swap by id
                                        // get operator
                                        // get swap router
    fn get_requests(
        e: Env,
        destination: Address,
    ) -> Vec<(BytesN<32>, u128, Address, i128, Address)>;
    fn get_completed_requests_last_page(e: Env, destination: Address) -> u32;
    fn get_completed_requests(
        e: Env,
        destination: Address,
        page: u32,
    ) -> Vec<(BytesN<32>, u128, Address, i128, Address, i128)>;
    fn get_destinations_last_page(e: Env) -> u32;
    fn get_destinations(e: Env, page: u32) -> Vec<Address>;

    fn get_operational_fee(e: Env) -> i128;
    fn set_operational_fee(e: Env, operator: Address, fee: i128);
}

pub trait UpgradeableContract {
    // Get contract version
    fn version() -> u32;

    // Upgrade contract with new wasm code
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>);
}
