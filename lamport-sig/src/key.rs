use bigint::U256;
use rand::Rng;
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use std::iter::repeat;

#[derive(Debug, Clone)]
pub struct PrivateKey {
    pub pairs: Vec<(U256, U256)>
}

pub static PRIVATE_KEY_LENGT: usize = 256;

impl PrivateKey {
    pub fn new() -> PrivateKey {
        let mut pairs = Vec::with_capacity(PRIVATE_KEY_LENGT);
        for _i in 0..PRIVATE_KEY_LENGT {
            pairs.push(random_uint256_pair());
        }
        PrivateKey {
            pairs: pairs
        }
    }
}

fn random_uint256_pair() -> (U256, U256) {
    (random_uint256(), random_uint256())
}

fn random_uint256() -> U256 {
    u64_to_uint256() * u64_to_uint256() * u64_to_uint256() * u64_to_uint256()
}

fn u64_to_uint256() -> U256 {
    let mut rng = rand::thread_rng();
    (rng.gen::<u64>()).into()
}

fn sha256_hash(random_number: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&random_number);
    sha256.result_str()
}

#[cfg(test)]

mod tests {
    use super::*;
    use std::any::type_name;

    #[test]
    fn test_new_block_chain() {
        let u64_to_uint256 = u64_to_uint256();
        assert_eq!(type_of(&U256), type_of(&u64_to_uint256));
    }

    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }
}