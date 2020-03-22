use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    input: Vec<Input>,
    output: Vec<Output>
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    transaction: Transaction,
    output_index: u8
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Output {
    recipient: String,
    amount: u64
}

impl Transaction {
    pub fn new() -> Transaction {
        Transaction {
            input: Vec::new(),
            output: Vec::new()
        }
    }

    pub fn admin_transfer(&mut self, recipient: &str) -> String {
        let output = Output {
            recipient: recipient.to_string(),
            amount: 100
        };
        self.output.push(output.clone());
        transactions_hash(&json!(output).to_string())
    }

    // pub fn transfer(&mut self, sender: &str, recipient: &str, amount) {

    // }
}

pub fn transactions_hash(transactions: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transactions);
    sha256.result_str()
}
