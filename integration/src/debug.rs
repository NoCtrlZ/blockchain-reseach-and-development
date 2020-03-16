use std::any::type_name;
use crate::blockchain;

pub fn type_of<T>(_: T) {
    println!("{:?}", type_name::<T>());
}

pub fn print_blockchain(blockchain: blockchain::Blockchain) {
    println!("blockchain: {:?}", blockchain);
}
