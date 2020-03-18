extern crate rand;
extern crate bigint;
extern crate rustc_hex;
extern crate base64;

mod lamport;
use lamport::Wallet;

fn main() {
    let plain_text = "secret message";
    let wallet = Wallet::new();
    let signature = wallet.sign(&plain_text);
    let public_key = wallet.get_public_key();
    let address = wallet.get_address();
    let is_valid = public_key.verify(plain_text, signature);
}
