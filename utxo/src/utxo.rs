use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Utxo {
    transactions: HashMap<String, Transaction>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    is_spent: bool,
    input: Vec<Input>,
    output: Vec<Output>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Input {
    transaction: Transaction,
    output_index: u8
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Output {
    recipient: String,
    amount: u64
}

impl Utxo {
    pub fn new() -> Utxo {
        Utxo {
            transactions: HashMap::new()
        }
    }

    pub fn admin_transfer(&mut self, recipient: &str) -> String {
        let input = Input {
            transaction: Transaction {
                is_spent: true,
                input: Vec::new(),
                output: Vec::new()
            },
            output_index: 0
        };
        let output = Output {
            recipient: recipient.to_string(),
            amount: 100
        };
        let mut transaction = Transaction {
            is_spent: false,
            input: Vec::new(),
            output: Vec::new()
        };
        transaction.input.push(input);
        transaction.output.push(output);
        let transaction_hash = transaction_hash(&json!(transaction).to_string());
        self.transactions.insert(transaction_hash.to_string(), transaction.clone());
        transaction_hash
    }

    // pub fn transfer(&mut self, sender: &str, recipient: &str, amount) {

    // }
}

pub fn transaction_hash(transaction: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transaction);
    sha256.result_str()
}
