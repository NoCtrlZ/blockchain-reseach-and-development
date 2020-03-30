use bigint::U256;
use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::iter::repeat;
use crate::unit::{add_zero, to_binary};
use base64::encode;

#[derive(Debug, Clone)]
pub struct Wallet {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
    pub address: String
}

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub pairs: Vec<(U256, U256)>
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub pairs: Vec<(U256, U256)>
}

pub static PRIVATE_KEY_LENGT: usize = 256;
pub static SIGNATURE_LENGT: usize = 256;

impl Wallet {
    pub fn new() -> Wallet {
        let mut prv_pairs = Vec::with_capacity(PRIVATE_KEY_LENGT);
        let mut pub_pairs = Vec::with_capacity(PRIVATE_KEY_LENGT);
        let mut public_key_string = "".to_string();
        for _i in 0..PRIVATE_KEY_LENGT {
            let (adam, eve) = prv_key_pair();
            pub_pairs.push(pub_key_pair(&adam, &eve));
            public_key_string.push_str(&encode(uint256_to_string(&adam, &eve).as_bytes()));
            prv_pairs.push((adam, eve));
        }
        let address = sha256_hash(&public_key_string).to_hex();
        Wallet {
            private_key: PrivateKey {
                pairs: prv_pairs
            },
            public_key: PublicKey {
                pairs: pub_pairs
            },
            address: address
        }
    }

    pub fn sign(&self, plain_text: &str) -> Vec<U256> {
        let mut message = message_creation(plain_text);
        let mut signature = Vec::with_capacity(SIGNATURE_LENGT);
        for i in 0..SIGNATURE_LENGT {
            match message.chars().nth(i).expect("fail to get char from message") {
                '1' => { signature.push(self.private_key.pairs[i].0) }
                '0' => { signature.push(self.private_key.pairs[i].1) }
                _ => panic!("this is not binary")
            }
        }
        signature
    }

    pub fn get_public_key(&self) -> PublicKey {
        self.public_key.clone()
    }

    pub fn get_address(&self) -> String {
        let mut address = "0x".to_string();
        address.push_str(&self.address.clone());
        address
    }

    pub fn string_public_key(&self) -> String {
        public_key_to_string(self.public_key.pairs.clone())
    }
}

impl PublicKey {
    pub fn verify(&self, plain_text: &str, signature: Vec<U256>) -> bool {
        let mut message = message_creation(plain_text);
        for i in 0..SIGNATURE_LENGT {
            match message.chars().nth(i).expect("fail to get char from message") {
                '1' => { if !compare_with_pub(signature[i], self.pairs[i].0) {panic!("invalid signature")}}
                '0' => { if !compare_with_pub(signature[i], self.pairs[i].1) {panic!("invalid signature")}}
                _ => panic!("this is not binary")
            }
        }
        true
    }
}

fn prv_key_pair() -> (U256, U256) {
    (random_uint256(), random_uint256())
}

fn random_uint256() -> U256 {
    u64_to_uint256() * u64_to_uint256() * u64_to_uint256() * u64_to_uint256()
}

fn u64_to_uint256() -> U256 {
    let mut rng = rand::thread_rng();
    (rng.gen::<u64>()).into()
}

fn pub_key_pair(adam: &U256, eve: &U256) -> (U256, U256) {
    (sha256_hash(&adam.to_string()), sha256_hash(&eve.to_string()))
}

fn sha256_hash(target: &str) -> U256 {
    let mut sha256 = Sha256::new();
    sha256.input_str(&target);
    from_str(&sha256.result_str())
}

fn message_creation(plain_text: &str) -> String {
    text_to_binary(&sha256_hash(&plain_text).to_string())
}

fn text_to_binary(hashed_text: &str) -> String {
    hashed_text.chars().map(to_binary).collect()
}

fn from_str(value: &str) -> U256 {
    use rustc_hex::FromHex;

    let bytes: Vec<u8> = match value.len() % 2 == 0 {
        true => value.from_hex().expect("fail to convert bytes to hex"),
        false => ("0".to_owned() + value).from_hex().expect("fail to add 0 to value")
    };

    let bytes_ref: &[u8] = &bytes;
    From::from(bytes_ref)
}

fn compare_with_pub(signature: U256, pub_key: U256) -> bool {
    sha256_hash(&signature.to_string()) == pub_key
}

fn uint256_to_string(adam: &U256, eve: &U256) -> String {
    format!("{}{}", adam.to_string(), eve.to_string())
}

fn public_key_to_string(public_key: Vec<(U256, U256)>) -> String {
    let mut string_key = "".to_string();
    for i in 0..PRIVATE_KEY_LENGT {
        string_key.push_str(&encode(public_key[i].0.to_hex()));
        string_key.push_str(&encode(public_key[i].1.to_hex()));
    }
    string_key
}

fn bigint_to_base64(target: U256) -> String {
    let hex = target.to_hex();
    let binary: String = hex.chars().map(to_binary).collect();
    let binary_256 = add_zero(&binary);
    "yo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn test_u64_to_bigint() {
        let u64_to_uint256 = u64_to_uint256();
        assert_eq!(type_of(U256), type_of(u64_to_uint256));
    }

    #[test]
    fn test_sha256_hash() {
        let hashed_value = sha256_hash("hello");
        assert_eq!(from_str("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"), hashed_value);
    }

    #[test]
    fn test_to_binary() {
        let text = "0123456789abcdef";
        let binary = text_to_binary(&text);
        assert_eq!("0000000100100011010001010110011110001001101010111100110111101111", &binary);
    }

    #[test]
    fn test_sign_message() {
        let text = "hello world";
        let wallet = Wallet::new();
        let signature = wallet.sign(&text);
        assert_eq!(SIGNATURE_LENGT, signature.len());
    }

    #[test]
    fn test_verify_signature() {
        let text = "hello world";
        let wallet = Wallet::new();
        let signature = wallet.sign(&text);
        let is_verify = wallet.public_key.verify(text, signature);
        assert_eq!(true, is_verify);
    }

    #[test]
    fn test_to_public_key() {
        let wallet= Wallet::new();
        let public_key = wallet.get_public_key();
        let mut public_key_pair = Vec::with_capacity(PRIVATE_KEY_LENGT);
        for i in 0..PRIVATE_KEY_LENGT {
            public_key_pair.push(pub_key_pair(&wallet.private_key.pairs[i].0, &wallet.private_key.pairs[i].1));
        }
        assert_eq!(public_key_pair, public_key.pairs);
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
}
