use crate::contracts::entrypoint_provider::EntryPointProvider;
use crate::db::dao::transaction_dao::TransactionDao;
use crate::models::transfer::status::Status::{FAILED, SUCCESS};
use crate::{CONFIG, PROVIDER};
use ethers::abi::RawLog;
use ethers::providers::Middleware;
use ethers::types::{Filter, H256};

pub async fn user_op_event_listener(
    transaction_dao: TransactionDao,
    entrypoint_provider: EntryPointProvider,
    user_op_hash: [u8; 32],
    txn_id: String,
) {
    let event = entrypoint_provider
        .abi()
        .event("UserOperationEvent")
        .unwrap();

    let filter = Filter::new()
        .address(CONFIG.get_chain().entrypoint_address)
        .topic0(event.signature())
        .topic1(H256::from(user_op_hash));

    let log_data = loop {
        let logs = PROVIDER.get_logs(&filter).await.unwrap();
        if logs.len() > 0 {
            break logs[0].clone();
        }
    };

    let txn_hash = format!("{:?}", log_data.transaction_hash.unwrap());

    let log = event
        .parse_log(RawLog {
            topics: log_data.topics,
            data: log_data.data.to_vec(),
        })
        .unwrap();

    let success_param = log
        .params
        .into_iter()
        .find(|param| param.name == "success")
        .unwrap();
    let success = success_param.value.into_bool().unwrap();

    let status = if success { SUCCESS } else { FAILED };

    transaction_dao
        .update_user_transaction(txn_id, txn_hash, status.to_string())
        .await;
}
