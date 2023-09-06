use ethers::abi::Address;
use ethers::contract::abigen;
use ethers::providers::{Http, Provider};
use ethers::types::{Bytes, U256};
use std::sync::Arc;

abigen!(SimpleAccount, "abi/SimpleAccount.json");

#[derive(Clone)]
pub struct SimpleAccountProvider {
    pub abi: SimpleAccount<Provider<Http>>,
}

impl SimpleAccountProvider {
    pub fn init_abi(
        client: Arc<Provider<Http>>,
        address: Address,
    ) -> SimpleAccount<Provider<Http>> {
        let contract: SimpleAccount<Provider<Http>> = SimpleAccount::new(address, client);
        contract
    }

    pub fn execute(&self, to: Address, value: String, data: Bytes) -> Result<Bytes, String> {
        let data = self
            .abi
            .execute(to, U256::from_dec_str(&value).unwrap(), data)
            .calldata();
        if data.is_none() {
            return Err("execute data failed".to_string());
        }

        Ok(data.unwrap())
    }
}
