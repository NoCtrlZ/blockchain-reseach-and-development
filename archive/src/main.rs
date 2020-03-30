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
