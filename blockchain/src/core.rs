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
    id: u64,
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

    // fn latest_block(&self) -> Block {
    //     self.entity[self.entity.len() - 1]
    // }

    pub fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) -> bool {
        let mut block = &mut self.entity[&mut self.entity.len() - 1];
        block.send_transaction(amount, sender, recipient)
    }

    pub fn print_latest_block(&self) {
        let block = &self.entity[self.entity.len() - 1];
        println!("index: {:?}", block.index);
        println!("timestamp: {:?}", block.timestamp);
        println!("transaction length: {:?}", block.transactions.len());
        println!("nonce: {:?}", block.nonce);
        println!("hash: {:?}", block.hash);
        println!("previous hash: {:?}", block.previous_hash);
    }

    // pub fn create_new_block(self) {
    //     let previous_hash = &self.entity[self.entity.len() - 1].hash;
    //     let current_hash = unit::block_hash(&self.entity[self.entity.len()]);

    // }
}

impl Block {
    fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) -> bool {
        let transaction = Transaction {
            id: self.transactions.len() as u64 + 1,
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        };
        self.transactions.push(transaction);
        true
    }

    fn print_latest_transaction(self) {
        let transaction = &self.transactions[self.transactions.len() - 1];
        println!("id: {:?}", transaction.id);
        println!("amount: {:?}", transaction.amount);
        println!("sender: {:?}", transaction.sender);
        println!("recipient: {:?}", transaction.recipient);
    }
}