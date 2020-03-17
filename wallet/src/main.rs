extern crate rsa;
extern crate rand;

mod debug;

use crate::debug::type_of;
use rsa::{PublicKey, RSAPrivateKey, PaddingScheme};
use rand::rngs::OsRng;
use std::str;

#[derive(Debug)]
struct PrivateKey {
    rng: OsRng,
    bits: usize,
    key: RSAPrivateKey
}

fn generate_key() -> PrivateKey {
    let mut rng = OsRng;
    let bits = 2048;
    let key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    PrivateKey {
        rng: rng,
        bits: bits,
        key: key
    }
}

fn main() {
    let mut private_key = generate_key();

    let data = b"hello world";
    let enc_data = private_key.key.encrypt(&mut private_key.rng, PaddingScheme::PKCS1v15, &data[..]).expect("failed to encrypt");
    let dec_data = private_key.key.decrypt(PaddingScheme::PKCS1v15, &enc_data).expect("failed to decrypt");
    println!("{:?}", str::from_utf8(&dec_data).unwrap());
}
