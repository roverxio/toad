use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use ethers::types::U256;
use ethers::utils::format_ether;
use log::error;
use std::sync::Arc;

use crate::models::contract_interaction;
use crate::provider::web3_client::Web3Client;

abigen!(VerifyingPaymaster, "abi/VerifyingPaymaster.json");

pub struct VerifyingPaymasterProvider;

impl VerifyingPaymasterProvider {
    pub fn init_abi(
        address: Address,
        client: Arc<Provider<Http>>,
    ) -> VerifyingPaymaster<Provider<Http>> {
        let contract: VerifyingPaymaster<Provider<Http>> = VerifyingPaymaster::new(address, client);
        contract
    }

    pub async fn get_deposit(client: &Web3Client) -> Result<String, String> {
        let response = client
            .get_verifying_paymaster_provider()
            .get_deposit()
            .await;
        if response.is_err() {
            error!(
                "Paymaster: Deposit: {:?}",
                response.err().unwrap().to_string()
            );
            return Err(String::from("Failed to get balance"));
        }
        Ok(format_ether(response.unwrap()))
    }

    pub fn get_verifying_paymaster_user_operation_payload(
        user_op: contract_interaction::user_operation::UserOperation,
    ) -> UserOperation {
        UserOperation {
            sender: user_op.sender,
            nonce: U256::from(user_op.nonce),
            init_code: user_op.init_code,
            call_data: user_op.calldata,
            call_gas_limit: U256::from(user_op.call_gas_limit),
            verification_gas_limit: U256::from(user_op.verification_gas_limit),
            pre_verification_gas: U256::from(user_op.pre_verification_gas),
            max_fee_per_gas: U256::from(user_op.max_fee_per_gas),
            max_priority_fee_per_gas: U256::from(user_op.max_priority_fee_per_gas),
            signature: user_op.signature,
            paymaster_and_data: user_op.paymaster_and_data,
        }
    }
}
