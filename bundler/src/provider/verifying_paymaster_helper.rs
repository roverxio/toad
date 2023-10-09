use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::U256;
use std::sync::Arc;

use crate::models::contract_interaction::user_operation;
use crate::CONFIG;

abigen!(VerifyingPaymaster, "abi/VerifyingPaymaster.json");

pub fn get_verifying_paymaster_abi(
    current_chain: &str,
    client: Arc<Provider<Http>>,
) -> VerifyingPaymaster<Provider<Http>> {
    let contract: VerifyingPaymaster<Provider<Http>> = VerifyingPaymaster::new(
        CONFIG.chains[current_chain].verifying_paymaster_address,
        client,
    );
    contract
}

pub fn get_verifying_paymaster_user_operation_payload(
    user_op: user_operation::UserOperation,
) -> UserOperation {
    UserOperation {
        sender: user_op.sender,
        nonce: U256::from(user_op.nonce),
        init_code: user_op.init_code,
        call_data: user_op.call_data,
        call_gas_limit: (user_op.call_gas_limit),
        verification_gas_limit: (user_op.verification_gas_limit),
        pre_verification_gas: (user_op.pre_verification_gas),
        max_fee_per_gas: (user_op.max_fee_per_gas),
        max_priority_fee_per_gas: (user_op.max_priority_fee_per_gas),
        signature: user_op.signature,
        paymaster_and_data: user_op.paymaster_and_data,
    }
}
