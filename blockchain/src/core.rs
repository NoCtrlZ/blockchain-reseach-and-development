use crate::unit;

#[derive(Debug)]
pub struct Blockchain {
    pub entity: Vec<Block>,
    pub transactions: Vec<Transaction>
}

#[derive(Debug)]
pub struct Block {
    index: u32,
    timestamp: u64,
    transactions: Vec<Transaction>,
    nonce: u64,
    hash: String,
    previous_hash: String
}

#[derive(Debug)]
pub struct Transaction {
    id: u64,
    amount: u64,
    sender: String,
    recipient: String
}

impl Blockchain {
    pub fn new() -> Blockchain {
        let mut blockchain = Blockchain {
            entity: Vec::new(),
            transactions: Vec::new()
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
        let first_block = Block {
            index: *&self.entity.len() as u32 + 1,
            timestamp: unit::current_time(),
            transactions: Vec::new(),
            nonce: 0,
            hash: "genesis block".to_string(),
            previous_hash: "this is start".to_string()
        };
    }

    fn latest_block(&mut self) -> &mut Block {
        self.entity.last_mut().unwrap()
    }

    pub fn send_transaction(&mut self, amount: u64, sender: &str, recipient: &str) -> bool {
        let mut block = self.latest_block();
        let transaction = Transaction {
            id: self.transactions.len() as u64 + 1,
            amount: amount,
            sender: sender.to_string(),
            recipient: recipient.to_string()
        };
        self.transactions.push(transaction);
        true
    }

    pub fn print_latest_block(&self) {
        let block = self.entity.last().unwrap();
        println!("index: {:?}", block);
    }

    pub fn print_blockchain(&self) {
        println!("index: {:?}", self);
    }

    // pub fn create_new_block(self) {
    //     let previous_hash = &self.entity[self.entity.len() - 1].hash;
    //     let current_hash = unit::block_hash(&self.entity[self.entity.len()]);

    // }
}

impl Block {
    fn print_latest_transaction(self) {
        let transaction = &self.transactions[self.transactions.len() - 1];
        println!("id: {:?}", transaction.id);
        println!("amount: {:?}", transaction.amount);
        println!("sender: {:?}", transaction.sender);
        println!("recipient: {:?}", transaction.recipient);
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
}