use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug)]
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

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    left: String,
    right: String,
    parent: String,
    sibling: String,
    position: String,
    data: String,
    hash: String
}

impl Tree {
    fn new(transactions: Vec<String>) -> Tree {
        Tree {
            leaves: transactions.clone(),
            layer: transactions.clone(),
            root: "".to_string()
        }
    }

    fn build_tree(&mut self) -> String {
        while self.layer.len() != 1 {
            self.build_layer();
        }
        self.layer[0].clone()
    }

    fn build_layer(&mut self) {
        let mut new_layer = Vec::new();
        if self.layer.len() % 2 == 1 {
            self.layer.push(self.layer.last().unwrap().to_string());
        }

        for i in 0..self.layer.len() / 2 {
            let left = hash(&self.layer[i * 2]);
            let right = hash(&self.layer[(i * 2) + 1]);
            let parent = hash(&format!("{}{}", &left, &right).to_string());
            new_layer.push(parent);
        }
        self.layer = new_layer;
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
    let mut tree = Tree::new(leaves);
    let merkle_root = tree.build_tree();
    println!("{:?}", merkle_root);
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

fn hash(transaction: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transaction);
    sha256.result_str()
}
