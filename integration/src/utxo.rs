use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::collections::HashMap;
use crate::unit::current_time;

#[derive(Debug, Deserialize, Serialize)]
pub struct Utxo {
    transactions: HashMap<String, HashMap<String, Transaction>>
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    is_spent: bool,
    timestamp: u64,
    input: Vec<Input>,
    output: Vec<Output>
}

impl Utxo {
    pub fn new() -> Utxo {
        Utxo {
            transactions: HashMap::new()
        }
    }

    pub fn admin_transfer(&mut self, recipient: &str) -> Transaction {
        let input = Input {
            transaction: Transaction {
                is_spent: true,
                timestamp: current_time(),
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
            timestamp: current_time(),
            input: Vec::new(),
            output: Vec::new()
        };
        transaction.input.push(input);
        transaction.output.push(output);
        let transaction_hash = transaction_hash(&json!(transaction).to_string());
        self.transactions.entry(recipient.to_string()).or_insert_with(HashMap::new).insert(transaction_hash.to_string(), transaction.clone());
        transaction
    }

    pub fn transfer(&mut self, tx_hash: &str, output_index: u8, sender: &str, recipient: &str, amount: u64) -> String {
        // todo multiple transactions input
        if self.transactions.contains_key(sender) && self.transactions[tx_hash].contains_key(tx_hash) {
            if !self.transactions[sender][tx_hash].is_spent {
                if self.transactions[sender][tx_hash].output[output_index as usize].amount > amount {
                    let input = Input {
                        transaction: self.transactions[sender][tx_hash].clone(),
                        output_index: output_index
                    };
                    let output = Output {
                        recipient: recipient.to_string(),
                        amount: amount
                    };
                    let sendback = Output {
                        recipient: sender.to_string(),
                        amount: self.transactions[sender][tx_hash].output[output_index as usize].amount - amount
                    };
                    let mut transaction = Transaction {
                        is_spent: false,
                        timestamp: current_time(),
                        input: Vec::new(),
                        output: Vec::new()
                    };
                    transaction.input.push(input);
                    transaction.output.push(output);
                    transaction.output.push(sendback);
                    self.transactions.get_mut(sender).unwrap().get_mut(tx_hash).unwrap().is_spent = true;
                    let tx_hash = transaction_hash(&json!(transaction).to_string());
                    self.transactions.entry(recipient.to_string()).or_insert_with(HashMap::new).insert(tx_hash.to_string(), transaction.clone());
                    self.transactions.entry(sender.to_string()).or_insert_with(HashMap::new).insert(tx_hash.to_string(), transaction.clone());
                    return tx_hash
                } else {
                    panic!("not enough money");
                }
            } else {
                panic!("transaction is already used");
            }
        } else {
            panic!("there is no transaction history");
        }
    }

    pub fn balance(&self, address: &str) -> u64 {
        let mut balance: u64 = 0;
        if self.transactions.contains_key(address) {
            for (tx_hash, transaction) in &self.transactions[address] {
                if !transaction.is_spent {
                    for i in 0..transaction.output.len() {
                        if transaction.output[i].recipient == address {
                            balance += transaction.output[i].amount;
                        }
                    }
                }
            }
        }
        balance
    }
}

pub fn transaction_hash(transaction: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transaction);
    sha256.result_str()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_transfer() {
        let dummy_address = "0x114514";
        let mut utxo = Utxo::new();
        let tx_hash = utxo.admin_transfer(dummy_address);
        let balance = utxo.balance(dummy_address);
        assert_eq!(100, balance);
    }

    #[test]
    fn test_transfer() {
        let dummy_address1 = "0x114514";
        let dummy_address2 = "0x114515";
        let mut utxo = Utxo::new();
        let tx_hash = utxo.admin_transfer(dummy_address1);
        let transfer_hash = utxo.transfer(&tx_hash, 0, &dummy_address1, &dummy_address2, 50);
        let balance1 = utxo.balance(dummy_address1);
        let balance2 = utxo.balance(dummy_address2);
        assert_eq!(50, balance1);
        assert_eq!(50, balance2);
    }

    #[test]
    #[should_panic]
    fn test_multiple_payment_fail() {
        let dummy_address = "0x114514";
        let mut utxo = Utxo::new();
        let tx_hash = utxo.admin_transfer(dummy_address);
        let transfer_hash = utxo.transfer(&tx_hash, 0, &dummy_address, "0x114515", 50);
        let transfer_hash = utxo.transfer(&tx_hash, 0, &dummy_address, "0x114515", 50);
    }

    #[test]
    #[should_panic]
    fn test_transfer_more_than_balance() {
        let dummy_address = "0x114514";
        let mut utxo = Utxo::new();
        let tx_hash = utxo.admin_transfer(dummy_address);
        let transfer_hash = utxo.transfer(&tx_hash, 0, &dummy_address, "0x114515", 5000);
    }
}
