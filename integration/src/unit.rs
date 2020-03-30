use std::time::{SystemTime, UNIX_EPOCH};
use crypto::sha2::Sha256;
use crypto::digest::Digest;
use rand::Rng;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::blockchain::Block;

pub fn current_time() -> u64 {
    let now = SystemTime::now();
    let unixtime = now.duration_since(UNIX_EPOCH).expect("invalid time");
    unixtime.as_secs()
}

pub fn transactions_hash(transactions: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&transactions);
    sha256.result_str()
}

pub fn sha256_hash(previous_block_hash: &str, current_block_hash: &str, nonce: &str) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(&format!("{}{}{}", previous_block_hash, current_block_hash, nonce));
    sha256.result_str()
}

pub fn random_port() -> String {
    let mut rng = rand::thread_rng();
    rng.gen_range(1024, 9000).to_string()
}

pub fn is_open(endpoint: &str) -> bool {
    match TcpListener::bind(&endpoint) {
        Ok(_) => false,
        Err(_) => true,
    }
}

pub fn stream_to_string(mut stream: TcpStream) -> String {
    let mut buffer = Vec::new();
    stream.read_to_end(&mut buffer).expect("fail to read stream to end");
    String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string()
}

pub fn difficulty_checker(difficulty: u8) -> String {
    let mut start_with = "".to_string();
    for _i in 0..difficulty {
        start_with.push_str("0");
    }
    start_with
}

pub fn block_is_valid(blocks: Vec<Block>) -> bool {
    for i in 1..blocks.len() {
        let start_with = difficulty_checker(blocks[i].difficulty);
        let hash = sha256_hash(&blocks[i].hash, &blocks[i].previous_hash, &blocks[i].nonce.to_string());
        if start_with != &hash[..blocks[i].difficulty as usize] {
            return false;
        }
    }
    true
}
