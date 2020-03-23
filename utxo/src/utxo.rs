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

    pub fn transfer(&mut self, tx_hash: &str, output_index: u8, sender: &str, recipient: &str, amount: u64) -> String {
        // todo multiple transactions input
        if !self.transactions[tx_hash].is_spent {
            if self.transactions[tx_hash].output[output_index as usize].amount > amount {
                let input = Input {
                    transaction: self.transactions[tx_hash].clone(),
                    output_index: output_index
                };
                let output = Output {
                    recipient: recipient.to_string(),
                    amount: amount
                };
                let sendback = Output {
                    recipient: sender.to_string(),
                    amount: self.transactions[tx_hash].output[output_index as usize].amount - amount
                };
                let mut transaction = Transaction {
                    is_spent: false,
                    input: Vec::new(),
                    output: Vec::new()
                };
                transaction.input.push(input);
                transaction.output.push(output);
                transaction.output.push(sendback);
                self.transactions.get_mut(tx_hash).unwrap().is_spent = true;
                let transaction_hash = transaction_hash(&json!(transaction).to_string());
                self.transactions.insert(transaction_hash.to_string(), transaction.clone());
                return transaction_hash
            } else {
                panic!("not enough money");
            }
        } else {
            panic!("do not exist transaction");
        }
    }
}

pub fn transaction_hash(transaction: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transaction);
    sha256.result_str()
}
