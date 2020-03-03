use crate::unit;

pub struct Blockchain {
    pub entity: Vec<Block>
}

pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: String,
    previous_hash: String
}

pub struct Transaction {
    amount: u64,
    sender: String,
    recipient: String
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            entity: Vec::new()
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let block = Block {
            index: 0,
            timestamp: unit::current_time(),
            transactions: Vec::new(),
            nonce: 0,
            hash: "genesis block".to_string(),
            previous_hash: "this is start".to_string()
        };
        self.entity.push(block);
    }

    pub fn print_latest_block(self) {
        let block = &self.entity[self.entity.len() - 1];
        println!("index: {:?}", block.index);
        println!("timestamp: {:?}", block.timestamp);
        println!("transaction length: {:?}", block.transactions.len());
        println!("nonce: {:?}", block.nonce);
        println!("hash: {:?}", block.hash);
        println!("previous hash: {:?}", block.previous_hash);
    }
}