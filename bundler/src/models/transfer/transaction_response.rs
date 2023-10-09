use serde::Serialize;

use crate::models::transfer::Status;

#[derive(Serialize)]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub status: String,
    pub explorer: String,
}

impl TransactionResponse {
    pub fn new(transaction_hash: String, status: Status, explorer: String) -> TransactionResponse {
        TransactionResponse {
            transaction_hash,
            status: status.to_string(),
            explorer,
        }
    }
}
