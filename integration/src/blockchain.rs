use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::unit::{current_time, transactions_hash, sha256_hash};
use crate::response;
use crate::request;
use crate::debug::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Blockchain {
    pub entity: Vec<Block>,
    pub transactions: Vec<Transaction>,
    pub difficulty: u8
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: String,
    previous_hash: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transaction {
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

    pub fn create_genesis_block(&mut self) {
        let block = Block {
            index: 0,
            timestamp: current_time(),
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
        transactions_hash(&transactions[0].to_string())
    }

    pub fn blockchain_json(&self) -> String {
        // println!("blockchain json");
        let blockchain = json!(&self);
        // println!("blockchain json done");
        blockchain.to_string()
    }

    pub fn proof_of_work(&mut self) -> Block {
        // println!("proof of work");
        let current_block_hash = self.block_hash();
        // println!("block hash");
        let previous_block_hash = self.latest_block_hash();
        // println!("latest block hash");
        let mut nonce: u64 = 0;
        let start_with = self.difficulty_checker();
        loop {
            let hash = sha256_hash(&current_block_hash, &previous_block_hash, &nonce.to_string());
            if start_with == &hash[..self.difficulty as usize] {
                break;
            }
            nonce+=1;
        }
        self.create_new_block(nonce)
    }

    pub fn create_new_block(&mut self, nonce: u64) -> Block {
        let block = Block {
            index: *&self.entity.len() as u32,
            timestamp: current_time(),
            transactions: self.transactions.clone(),
            nonce: nonce,
            hash: self.block_hash(),
            previous_hash: (&self.latest_block_hash()).to_string()
        };
        let block_log = block.clone();
        self.entity.push(block);
        self.transactions.clear();
        block_log
    }

    pub fn difficulty_checker(&self) -> String {
        let mut start_with = "".to_string();
        for _i in 0..self.difficulty {
            start_with.push_str("0");
        }
        start_with
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_block_chain() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.entity[0].index, 0);
        assert_eq!(blockchain.entity[0].nonce, 0);
        assert_eq!(blockchain.entity[0].transactions.len(), 0);
        assert_eq!(blockchain.entity[0].hash, "genesis block".to_string());
        assert_eq!(blockchain.entity[0].previous_hash, "this is start".to_string());
    }

    #[test]
    fn test_get_previous_hash() {
        let blockchain = Blockchain::new();
        let previous_hash = blockchain.latest_block_hash();
        assert_eq!(blockchain.entity[0].previous_hash, previous_hash);
    }

    #[test]
    fn test_difficulty_checker() {
        let blockchain = Blockchain::new();
        let start_with = blockchain.difficulty_checker();
        assert_eq!(start_with, "000");
    }
}
