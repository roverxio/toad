use log::info;

use crate::CONFIG;
use crate::constants::Constants;
use crate::errors::ApiError;
use crate::models::admin::paymaster_topup::PaymasterTopup;
use crate::models::transfer::transfer_response::TransactionResponse;
use crate::models::wallet::balance_request::Balance;
use crate::models::wallet::balance_response::BalanceResponse;
use crate::provider::paymaster_provider::PaymasterProvider;

#[derive(Clone)]
pub struct AdminService {
    pub paymaster_provider: PaymasterProvider,
}

impl AdminService {
    pub fn topup_paymaster_deposit(
        &self,
        topup: PaymasterTopup,
    ) -> Result<TransactionResponse, ApiError> {
        info!("topup: {:?}", topup.address);
        Ok(TransactionResponse {
            transaction_hash: "hash".to_string(),
            status: "success".to_string(),
            explorer: "no".to_string(),
        })
    }

    pub async fn get_balance(&self, entity: String, data: Balance) -> Result<BalanceResponse, ApiError> {
        if data.currency != Constants::NATIVE {
            return Err(ApiError::BadRequest("Invalid currency".to_string()));
        }
        if Constants::PAYMASTER == entity {
            let paymaster_address = &CONFIG.chains[&CONFIG.run_config.current_chain].verifying_paymaster_address;
            let result = self.paymaster_provider.get_deposit().await;
            if result.is_err() {
                return Err(ApiError::InternalServer(result.err().unwrap()));
            }
            return Ok(BalanceResponse {
                balance: result.unwrap(),
                address: format!("{:?}", paymaster_address),
                currency: data.currency,
            });
        }
        Err(ApiError::BadRequest("Invalid entity".to_string()))
    }
}
