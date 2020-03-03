use crate::core;
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn current_time() -> u64 {
    let now = SystemTime::now();
    let unixtime = now.duration_since(UNIX_EPOCH).expect("invalid time");
    unixtime.as_secs()
}

// pub fn block_hash(block: core::Block) -> String {
//     let stringified_transactions = transactions_to_string(block.transactions); 
// }

pub fn proof_of_work(block: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&block);
    sha256.result_str()
}

// pub fn transactions_to_string(transactions: core::Transaction) -> String {
//     for (i, transaction) in transactions.enumerate() {
//         transaction.id;
//     }
// }
