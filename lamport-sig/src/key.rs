use bigint::U256;
use rand::Rng;

#[derive(Debug)]
pub struct PrivateKey {
    pairs: Vec<(U256, U256)>
}

impl PrivateKey {
    pub fn new() -> PrivateKey {
        let secret_key_length: usize = 256;
        let mut pairs = Vec::with_capacity(secret_key_length);
        for _i in 0..secret_key_length {
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
    u64_to_uint256() * u64_to_uint256()
}

fn u64_to_uint256() -> U256 {
    let mut rng = rand::thread_rng();
    (rng.gen::<u64>()).into()
}

#[cfg(test)]
use std::any::type_name;

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