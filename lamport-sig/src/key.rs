use bigint::U256;
use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::iter::repeat;

#[derive(Debug)]
pub struct PrivateKey {
    pub pairs: Vec<(U256, U256)>,
    pub public_key: PublicKey
}

#[derive(Debug)]
pub struct PublicKey {
    pub pairs: Vec<(String, String)>
}

pub static PRIVATE_KEY_LENGT: usize = 256;

impl PrivateKey {
    pub fn new() -> PrivateKey {
        let mut prv_pairs = Vec::with_capacity(PRIVATE_KEY_LENGT);
        let mut pub_pairs = Vec::with_capacity(PRIVATE_KEY_LENGT);
        for _i in 0..PRIVATE_KEY_LENGT {
            let (adam, eve) = prv_key_pair();
            pub_pairs.push(pub_key_pair(&adam, &eve));
            prv_pairs.push((adam, eve));
        }
        let public_key = PublicKey {
            pairs: pub_pairs
        };
        PrivateKey {
            pairs: prv_pairs,
            public_key: public_key
        }
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

fn pub_key_pair(adam: &U256, eve: &U256) -> (String, String) {
    (sha256_hash(&adam.to_string()), sha256_hash(&eve.to_string()))
}

fn sha256_hash(random_number: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&random_number);
    sha256.result_str()
}

fn text_to_binary(plain_text: &str) -> String {
    plain_text.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'a' => "1010",
        'b' => "1011",
        'c' => "1100",
        'd' => "1101",
        'e' => "1110",
        'f' => "1111",
        _ => "",
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn test_u64_to_bigint() {
        let u64_to_uint256 = u64_to_uint256();
        assert_eq!(type_of(&U256), type_of(&u64_to_uint256));
    }

    #[test]
    fn test_sha256_hash() {
        let hashed_value = sha256_hash("hello");
        assert_eq!("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824", &hashed_value);
        assert_eq!(64, hashed_value.len());
    }

    #[test]
    fn test_to_binary() {
        let text = "0123456789abcdef";
        let binary = text_to_binary(&text);
        assert_eq!("0000000100100011010001010110011110001001101010111100110111101111", &binary);
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
}