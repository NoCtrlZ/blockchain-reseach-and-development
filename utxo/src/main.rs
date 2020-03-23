mod utxo;

use utxo::Utxo;

fn main() {
    let dummy_address = "0x114514";
    let mut utxo = Utxo::new();
    let tx_hash = utxo.admin_transfer(dummy_address);
    println!("admin transfer tx hash is {}", &tx_hash);
    let transfer_hash = utxo.transfer(&tx_hash, 0, &dummy_address, "0x114515", 50);
    println!("transfer tx hash is {}", transfer_hash);
}
