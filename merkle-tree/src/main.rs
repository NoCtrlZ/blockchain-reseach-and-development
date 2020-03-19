use serde_json::json;
use serde::{Deserialize, Serialize};

struct Tree {
    leaves: Vec<String>,
    layer: Vec<String>,
    root: String
}

#[derive(Debug)]
struct Transactions {
    transactions: Vec<Transaction>
}

#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    amount: u64,
    sender: String,
    recipient: String
}

impl Tree {
    fn new(transactions: Vec<String>) -> Tree {
        Tree {
            leaves: transactions.clone(),
            layer: transactions.clone(),
            root: "".to_string()
        }
    }
}

impl Transactions {
    fn new() -> Transactions {
        Transactions {
            transactions: Vec::new()
        }
    }

    fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) {
        self.transactions.push(Transaction {
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        })
    }

    fn transactions_to_leaves(&self) -> Vec<String> {
        let mut leaves = Vec::new();
        for i in 0..self.transactions.len() {
            leaves.push(json!(self.transactions[i]).to_string());
        }
        leaves
    }
}

fn main() {
    let mut transactions = Transactions::new();
    send_transactions(&mut transactions);
    let leaves = transactions.transactions_to_leaves();
    println!("{:?}", leaves);
}

fn send_transactions(transactions: &mut Transactions) {
    transactions.send_transaction(100, "alice", "bob");
    transactions.send_transaction(50, "alice", "bob");
    transactions.send_transaction(25, "alice", "bob");
    transactions.send_transaction(12, "alice", "bob");
    transactions.send_transaction(100, "alice", "crea");
    transactions.send_transaction(50, "alice", "crea");
    transactions.send_transaction(25, "alice", "crea");
    transactions.send_transaction(12, "alice", "crea");
    transactions.send_transaction(100, "bod", "crea");
    transactions.send_transaction(100, "leon", "jack");
}