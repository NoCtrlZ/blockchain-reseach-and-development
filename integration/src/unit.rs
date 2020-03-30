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

pub fn to_binary(c: char) -> String {
    match c {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'a' => "1010".to_string(),
        'b' => "1011".to_string(),
        'c' => "1100".to_string(),
        'd' => "1101".to_string(),
        'e' => "1110".to_string(),
        'f' => "1111".to_string(),
        _ => "".to_string(),
    }
}

pub fn to_base64(binary: &str) -> String {
    let mut target = "".to_string();
    let length = binary.len();
    let head = 6 - length;
    if length > 6 {
        panic!("give me string less than six");
    } else {
        for i in 0..head {
            target.push_str("0");
        }
        target.push_str(binary);
    }
    let mut mark = match target.as_str() {
        "000000" => "A".to_string(),
        "000001" => "B".to_string(),
        "000010" => "C".to_string(),
        "000011" => "D".to_string(),
        "000100" => "E".to_string(),
        "000101" => "F".to_string(),
        "000110" => "G".to_string(),
        "000111" => "H".to_string(),
        "001000" => "I".to_string(),
        "001001" => "J".to_string(),
        "001010" => "K".to_string(),
        "001011" => "L".to_string(),
        "001100" => "M".to_string(),
        "001101" => "N".to_string(),
        "001110" => "O".to_string(),
        "001111" => "P".to_string(),
        "010000" => "Q".to_string(),
        "010001" => "R".to_string(),
        "010010" => "S".to_string(),
        "010011" => "T".to_string(),
        "010100" => "U".to_string(),
        "010101" => "V".to_string(),
        "010110" => "W".to_string(),
        "010111" => "X".to_string(),
        "011000" => "Y".to_string(),
        "011001" => "Z".to_string(),
        "011010" => "a".to_string(),
        "011011" => "b".to_string(),
        "011100" => "c".to_string(),
        "011101" => "d".to_string(),
        "011110" => "e".to_string(),
        "011111" => "f".to_string(),
        "100000" => "g".to_string(),
        "100001" => "h".to_string(),
        "100010" => "i".to_string(),
        "100011" => "j".to_string(),
        "100100" => "k".to_string(),
        "100101" => "l".to_string(),
        "100110" => "m".to_string(),
        "100111" => "n".to_string(),
        "101000" => "o".to_string(),
        "101001" => "p".to_string(),
        "101010" => "q".to_string(),
        "101011" => "r".to_string(),
        "101100" => "s".to_string(),
        "101101" => "t".to_string(),
        "101110" => "u".to_string(),
        "101111" => "v".to_string(),
        "110000" => "w".to_string(),
        "110001" => "x".to_string(),
        "110010" => "y".to_string(),
        "110011" => "z".to_string(),
        "110100" => "0".to_string(),
        "110101" => "1".to_string(),
        "110110" => "2".to_string(),
        "110111" => "3".to_string(),
        "111000" => "4".to_string(),
        "111001" => "5".to_string(),
        "111010" => "6".to_string(),
        "111011" => "7".to_string(),
        "111100" => "8".to_string(),
        "111101" => "9".to_string(),
        "111110" => "+".to_string(),
        "111111" => "/".to_string(),
        _ => panic!("fail to convert"),
    };
    for i in 0..head {
        mark.push_str("=");
    }
    mark
}

pub fn add_zero(binary: &str) -> String {
    let zero_num = 256 - binary.len();
    let mut prefix = "".to_string();
    for i in 0..zero_num {
        prefix.push_str("0");
    }
    format!("{}{}", prefix, binary)
}

pub fn binary_to_base64(binary: &str) -> String {
    let mut base64 = "".to_string();
    let iter_num = binary.len() / 6;
    for i in 0..iter_num {
        let piece: String = match i != iter_num - 1 {
            true => binary[6 * i..(6 * i) + 6].to_string(),
            false => binary[6 * i..(6 * i) + 4].to_string(),
        };
        base64.push_str(&to_base64(piece.as_str()));
    }
    base64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_zero() {
        let binary = "101010100101001010010101001010100101010010101001010100101001010010101010101010";
        let added_zero = add_zero(binary);
        assert_eq!(added_zero.len(), 256);
    }

    #[test]
    fn test_binary_to_base64() {
        let binary = "101010100101001010010101001010100101010010101001010100101001010010101010101010";
        let added_zero = add_zero(binary);
        let base64 = binary_to_base64(&added_zero);
        println!("{:?}", base64);
    }

    #[test]
    fn test_to_base64() {
        let Z = to_base64("011001");
        let four = to_base64("111000");
        let with_postfix = to_base64("10110");
        assert_eq!(Z, "Z");
        assert_eq!(four, "4");
        assert_eq!(with_postfix, "W=");
    }
}
