use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::unit;

#[derive(Debug, Deserialize, Serialize)]
pub struct Blockchain {
    pub entity: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub difficulty: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u128,
    hash: String,
    previous_hash: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
    id: u64,
    amount: u64,
    sender: String,
    recipient: String
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let default_difficulty = 3;
        let mut blockchain = Blockchain {
            entity: Vec::new(),
            transactions: Vec::new(),
            difficulty: default_difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    fn create_genesis_block(&mut self) {
        let block = Block {
            index: 0,
            timestamp: unit::current_time(),
            transactions: [].to_vec(),
            nonce: 0,
            hash: "genesis block".to_string(),
            previous_hash: "this is start".to_string()
        };
        self.entity.push(block);
    }

    pub fn latest_block_hash(&self) -> &str {
        &self.entity.last().unwrap().previous_hash
    }

    pub fn block_hash(&self) -> String {
        let transactions = json!(self.transactions);
        unit::transactions_hash(&transactions[0].to_string())
    }

    pub fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) -> bool {
        let transaction = Transaction {
            id: self.transactions.len() as u64 + 1,
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        };
        self.transactions.push(transaction);
        true
    }

    pub fn proof_of_work(&mut self) {
        let difficulty = self.difficulty;
        let current_block_hash = self.block_hash();
        let previous_block_hash = self.latest_block_hash();
        let mut start_with = "".to_string();
        for _i in 0..difficulty {
            start_with.push_str("0");
        }
        let mut nonce: u128 = 0;
        loop {
            let hash = unit::sha256_hash(&current_block_hash, &previous_block_hash, &nonce.to_string());
            if start_with == &hash[..difficulty as usize] {
                break;
            }
            nonce+=1;
        }
        self.create_new_block(nonce);
    }

    pub fn create_new_block(&mut self, nonce: u128) {
        let block = Block {
            index: *&self.entity.len() as u32,
            timestamp: unit::current_time(),
            transactions: self.transactions.clone(),
            nonce: nonce,
            hash: self.block_hash(),
            previous_hash: (&self.latest_block_hash()).to_string()
        };
        self.entity.push(block);
        self.transactions.clear();
    }

    pub fn print_blockchain(&self) {
        println!("blockchain: {:?}", self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_chain() {
        let mut blockchain = Blockchain::new();
        assert_eq!(blockchain.entity[0].index, 0);
        assert_eq!(blockchain.entity[0].nonce, 0);
        assert_eq!(blockchain.entity[0].transactions.len(), 0);
        assert_eq!(blockchain.entity[0].hash, "genesis block".to_string());
        assert_eq!(blockchain.entity[0].previous_hash, "this is start".to_string());
    }

    #[test]
    fn test_send_transaction() {
        let mut blockchain = Blockchain::new();
        let result = blockchain.send_transaction(100, "alice", "bob");
        assert_eq!(blockchain.transactions[0].amount, 100);
        assert_eq!(blockchain.transactions[0].sender, "alice");
        assert_eq!(blockchain.transactions[0].recipient, "bob");
        assert_eq!(result, true);
    }
}
