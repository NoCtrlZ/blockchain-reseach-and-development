use serde_json::json;
use serde::{Deserialize, Serialize};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::any::type_name;

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
    sibling: Box<Option<Node>>,
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
                sibling: Box::new(None),
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

    fn build_tree(&mut self) -> Node {
        while self.layer.len() != 1 {
            self.build_layer();
        }
        self.layer[0].clone()
    }

    fn build_layer(&mut self) {
        // println!("{:?}", self.layer.len() == self.leaves.len());
        let mut new_layer = Vec::new();
        if self.layer.len() % 2 == 1 {
            self.layer.push(self.layer.last().unwrap().clone());
        }
        for i in 0..self.layer.len() / 2 {
            let mut parent = Node {
                left: Box::new(None),
                right: Box::new(None),
                parent: Box::new(None),
                sibling: Box::new(None),
                position: "".to_string(),
                data: format!("{}{}", &self.layer[i].hash, &self.layer[i].hash).to_string(),
                hash: hash(&format!("{}{}", &self.layer[i].hash, &self.layer[i].hash).to_string())
            };
            self.assign_node(i);
            // println!("{:?}", left.sibling);
            parent.left = Box::new(Some(self.layer[(i * 2)].clone()));
            parent.right = Box::new(Some(self.layer[(i * 2) + 1].clone()));

            self.parent_assign(i, parent.clone());

            // println!("{:?}", left);
            new_layer.push(parent);
            if self.layer.len() == self.leaves.len() {
                if let Some(leave) = self.leaves.get_mut(i * 2) {
                    *leave = self.layer[(i * 2)].clone()
                }
                if let Some(leave) = self.leaves.get_mut((i * 2) + 1) {
                    *leave = self.layer[(i * 2) + 1].clone()
                }
                println!("{:?}", self.leaves)
            }
        }
        self.layer.clear();
        for i in 0..new_layer.len() {
            self.layer.push(new_layer[i].clone());
        };
    }

    fn parent_assign(&mut self, i: usize, parent: Node) {
        self.parent_assign_left(i, parent.clone());
        self.parent_assign_rigth(i, parent);
    }

    fn parent_assign_left(&mut self, i: usize, parent: Node) {
        let mut left = self.layer.get_mut(i * 2).unwrap();
        left.parent = Box::new(Some(parent));
    }

    fn parent_assign_rigth(&mut self, i: usize, parent: Node) {
        let mut right = self.layer.get_mut((i * 2) + 1).unwrap();
        right.parent = Box::new(Some(parent));
    }

    fn assign_node(&mut self, i: usize) {
        self.assign_left(i, self.get_node((i * 2) + 1));
        self.assign_right(i, self.get_node(i * 2));
    }

    fn assign_left(&mut self, i: usize, right_node: Node) {
        let mut left = self.layer.get_mut(i * 2).unwrap();
        left.sibling = Box::new(Some(right_node));
        left.position = "left".to_string();
    }

    fn assign_right(&mut self, i: usize, left_node: Node) {
        let mut right = self.layer.get_mut((i * 2) + 1).unwrap();
        right.sibling = Box::new(Some(left_node));
        right.position = "right".to_string();
    }

    fn get_node(&self, i: usize) -> Node {
        self.layer[(i)].clone()
    }

    fn search(&self, amount: u64, sender: &str, recipient: &str) -> Result<Node, bool> {
        let transaction = json!(Transaction {
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        }).to_string();
        for i in 0..self.leaves.len() {
            // println!("{:?}", self.leaves[i]);
            if hash(&transaction) == self.leaves[i].hash {
                return Ok(self.leaves[i].clone());
            }
        }
        Err(false)
    }

    fn get_pass(&self, amount: u64, sender: &str, recipient: &str) {
        let mut target = self.search(amount, sender, recipient).unwrap();
        println!("{:?}", &target);
        // let markle_pass = Vec::new();
        // loop {
            match *target.parent {
                Some(node) => {
                    println!("{:?}", node.clone());
                    let sibling = node.sibling;
                    // println!("{:?}", sibling);
                    // markle_pass.push((sibling.hash, sibling.position));
                    // target = target.parent;
                },
                None => {
                    // break;
                }
            }
        // }
        // markle_pass;
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
    // println!("{:?}", &tree);
    let merkle_root = tree.build_tree();
    // println!("{:?}", merkle_root);
    let target = tree.search(50, "alice", "bob").unwrap();
    println!("{:?}", target.clone());
    let t = tree.get_pass(50, "alice", "bob");
    // type_of(target.parent);
    println!("{:?}", t);
}

fn send_transactions(transactions: &mut Transactions) {
    transactions.send_transaction(100, "alice", "bob");
    transactions.send_transaction(50, "alice", "bob");
    transactions.send_transaction(25, "alice", "bob");
    transactions.send_transaction(12, "alice", "bob");
    // transactions.send_transaction(100, "alice", "crea");
    // transactions.send_transaction(50, "alice", "crea");
    // transactions.send_transaction(25, "alice", "crea");
    // transactions.send_transaction(12, "alice", "crea");
    // transactions.send_transaction(100, "bod", "crea");
    // transactions.send_transaction(100, "leon", "jack");
}

fn hash(transaction: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transaction);
    sha256.result_str()
}

pub fn type_of<T>(_: T) {
    println!("{:?}", type_name::<T>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_node() {
        let mut transactions = Transactions::new();
        transactions.send_transaction(100, "alice", "bob");
        transactions.send_transaction(50, "alice", "bob");
        let leaves = transactions.transactions_to_leaves();
        let mut tree = Tree::new(leaves);
        println!("{:?}", tree);
    }
}
