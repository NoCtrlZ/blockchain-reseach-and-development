use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

#[derive(Debug)]
struct Tree {
    leaves: Vec<Node>,
    layer: Vec<Node>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Node {
    left: Box<Option<Node>>,
    right: Box<Option<Node>>,
    parent: Box<Option<Node>>,
    sibling: String,
    position: String,
    data: String,
    hash: String
}

impl Tree {
    fn new(transactions: Vec<String>) -> Tree {
        let mut leaves = Vec::new();
        for i in 0..transactions.len() {
            leaves.push(Node {
                left: Box::new(None),
                right: Box::new(None),
                parent: Box::new(None),
                sibling: "".to_string(),
                position: "".to_string(),
                data: transactions[i].to_string(),
                hash: hash(&transactions[i]).to_string(),
            })
        }
        Tree {
            leaves: leaves.clone(),
            layer: leaves,
            root: "".to_string()
        }
    }

    fn build_tree(&mut self) -> String {
        while self.layer.len() != 1 {
            self.build_layer();
        }
        self.layer[0].hash.clone()
    }

    fn build_layer(&mut self) {
        let mut new_layer = Vec::new();
        if self.layer.len() % 2 == 1 {
            self.layer.push(self.layer.last().unwrap().clone());
        }

        for i in 0..self.layer.len() / 2 {
            let mut left = self.layer[i * 2].clone();
            let mut right = self.layer[(i * 2) + 1].clone();
            let parent = Node {
                left: Box::new(None),
                right: Box::new(None),
                parent: Box::new(None),
                sibling: "".to_string(),
                position: "".to_string(),
                data: format!("{}{}", &left.hash, &right.hash).to_string(),
                hash: hash(&format!("{}{}", &left.hash, &right.hash).to_string())
            };
            new_layer.push(parent);
        }
        self.layer = new_layer;
    }

    fn search(&self, amount: u64, sender: &str, recipient: &str) -> Result<Node, bool> {
        let transaction = json!(Transaction {
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        }).to_string();
        for i in 0..self.leaves.len() {
            if hash(&transaction) == self.leaves[i].hash {
                return Ok(self.leaves[i].clone());
            }
        }
        Err(false)
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
    let target = tree.search(25, "alice", "crea").unwrap();
    println!("{:?}", target);
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
