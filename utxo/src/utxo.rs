use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug, Deserialize, Serialize)]
pub struct Utxo {
    transactions: Vec<Transaction>
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
            transactions: Vec::new()
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
        self.transactions.push(transaction.clone());
        transactions_hash(&json!(transaction).to_string())
    }

    // pub fn transfer(&mut self, sender: &str, recipient: &str, amount) {

    // }
}

pub fn transactions_hash(transactions: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transactions);
    sha256.result_str()
}
