mod blockchain;
mod p2p;
mod unit;
mod debug;

extern crate rand;

#[tokio::main]
async fn main() {
    let network = p2p::Network::new();
    let addr = network.get_address();
    println!("{:?}", addr);

    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();

    debug::print_blockchain(blockchain);
}
