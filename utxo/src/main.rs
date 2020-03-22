mod utxo;

use utxo::Transaction;

fn main() {
    let utxo = Transaction::new();
    println!("{:?}", utxo);
}
