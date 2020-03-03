use std::time::{SystemTime, UNIX_EPOCH};
use crypto::sha2::Sha256;
use crypto::digest::Digest;

pub fn current_time() -> u64 {
    let now = SystemTime::now();
    let unixtime = now.duration_since(UNIX_EPOCH).expect("invalid time");
    unixtime.as_secs()
}

pub fn block_hash(block: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&block);
    sha256.result_str()
}
