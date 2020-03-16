mod blockchain;
mod p2p;
mod unit;

fn main() {
    let mut blockchain = blockchain::Blockchain::new();
    blockchain.send_transaction(100, "alice", "bob");
    let nonce = blockchain.proof_of_work();
    blockchain.print_blockchain();
}
