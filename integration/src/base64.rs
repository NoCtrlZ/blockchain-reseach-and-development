pub fn add_zero(binary: &str) -> String {
    let zero_num = 256 - binary.len();
    let mut prefix = "".to_string();
    for i in 0..zero_num {
        prefix.push_str("0");
    }
    format!("{}{}", prefix, binary)
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

pub fn binary_to_hex(binary: &str) -> String {
    let mut hex = "".to_string();
    let length = binary.len() / 4;
    for i in 0..length {
        hex.push_str(&binary_hexize(&binary[4 * i..(4 * i) + 4]));
    }
    hex
}

fn binary_hexize(binary: &str) -> String {
    match binary {
        "0000" => "0".to_string(),
        "0001" => "1".to_string(),
        "0010" => "2".to_string(),
        "0011" => "3".to_string(),
        "0100" => "4".to_string(),
        "0101" => "5".to_string(),
        "0110" => "6".to_string(),
        "0111" => "7".to_string(),
        "1000" => "8".to_string(),
        "1001" => "9".to_string(),
        "1010" => "a".to_string(),
        "1011" => "b".to_string(),
        "1100" => "c".to_string(),
        "1101" => "d".to_string(),
        "1110" => "e".to_string(),
        "1111" => "f".to_string(),
        _ => "".to_string(),
    }
}

pub fn binary_to_base64(binary: &str) -> String {
    let mut base64 = "".to_string();
    let iter_num = (binary.len() / 6) + 1;
    for i in 0..iter_num {
        let piece: String = match i != iter_num - 1 {
            true => binary[6 * i..(6 * i) + 6].to_string(),
            false => binary[6 * i..(6 * i) + 4].to_string(),
        };
        base64.push_str(&to_base64(piece.as_str()));
    }
    base64
}

fn to_base64(binary: &str) -> String {
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
    mark
}

fn base64_to_binary(base64: &str) -> String {
    if base64.len() > 43 {panic!("give me single string");}
    let mut binary = "".to_string();
    for i in 0..base64.len() {
        let mut mark = match base64.chars().nth(i).expect("fail to get base64") {
            'A' => "000000".to_string(),
            'B' => "000001".to_string(),
            'C' => "000010".to_string(),
            'D' => "000011".to_string(),
            'E' => "000100".to_string(),
            'F' => "000101".to_string(),
            'G' => "000110".to_string(),
            'H' => "000111".to_string(),
            'I' => "001000".to_string(),
            'J' => "001001".to_string(),
            'K' => "001010".to_string(),
            'L' => "001011".to_string(),
            'M' => "001100".to_string(),
            'N' => "001101".to_string(),
            'O' => "001110".to_string(),
            'P' => "001111".to_string(),
            'Q' => "010000".to_string(),
            'R' => "010001".to_string(),
            'S' => "010010".to_string(),
            'T' => "010011".to_string(),
            'U' => "010100".to_string(),
            'V' => "010101".to_string(),
            'W' => "010110".to_string(),
            'X' => "010111".to_string(),
            'Y' => "011000".to_string(),
            'Z' => "011001".to_string(),
            'a' => "011010".to_string(),
            'b' => "011011".to_string(),
            'c' => "011100".to_string(),
            'd' => "011101".to_string(),
            'e' => "011110".to_string(),
            'f' => "011111".to_string(),
            'g' => "100000".to_string(),
            'h' => "100001".to_string(),
            'i' => "100010".to_string(),
            'j' => "100011".to_string(),
            'k' => "100100".to_string(),
            'l' => "100101".to_string(),
            'm' => "100110".to_string(),
            'n' => "100111".to_string(),
            'o' => "101000".to_string(),
            'p' => "101001".to_string(),
            'q' => "101010".to_string(),
            'r' => "101011".to_string(),
            's' => "101100".to_string(),
            't' => "101101".to_string(),
            'u' => "101110".to_string(),
            'v' => "101111".to_string(),
            'w' => "110000".to_string(),
            'x' => "110001".to_string(),
            'y' => "110010".to_string(),
            'z' => "110011".to_string(),
            '0' => "110100".to_string(),
            '1' => "110101".to_string(),
            '2' => "110110".to_string(),
            '3' => "110111".to_string(),
            '4' => "111000".to_string(),
            '5' => "111001".to_string(),
            '6' => "111010".to_string(),
            '7' => "111011".to_string(),
            '8' => "111100".to_string(),
            '9' => "111101".to_string(),
            '+' => "111110".to_string(),
            '/' => "111111".to_string(),
            _ => panic!("fail to convert"),
        };
        if i == base64.len() - 1 {
            mark.remove(0);
            mark.remove(1);
        }
        binary.push_str(&mark);
    }
    binary
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_to_hex() {
        let base64 = "CpESkZ0G4GgV+qRV9dO7MteCsOWy3TMSkVaJvXhc8WA";
        let binary = base64_to_binary(base64);
        let hex = binary_to_hex(&binary);
        assert_eq!(hex.len(), 64);
    }

    #[test]
    fn test_base64_to_binary() {
        let base64 = "CpESkZ0G4GgV+qRV9dO7MteCsOWy3TMSkVaJvXhc8WA";
        let binary = base64_to_binary(base64);
        assert_eq!(binary.len(), 256);
    }

    #[test]
    #[should_panic]
    fn test_base64_panic() {
        let base64 = "CpESkZ0G4GgV+qRV9dO7MteCsOWy3TMSkVaJvXhc8WA7";
        let binary = base64_to_binary(base64);
    }

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
        assert_eq!(base64.len(), 43);
    }

    #[test]
    fn test_to_base64() {
        let Z = to_base64("011001");
        let four = to_base64("111000");
        let with_postfix = to_base64("10110");
        assert_eq!(Z, "Z");
        assert_eq!(four, "4");
        assert_eq!(with_postfix, "W");
    }
}
