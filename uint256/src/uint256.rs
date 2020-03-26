#[derive(Debug)]
pub struct u256 (
    pub [u64; 4]
);

impl u256 {
    pub const MAX_VALUE: u256 = u256([u64::max_value(); 4]);
}

impl From<u64> for u256 {
    fn from(number: u64) -> Self {
        let mut numbers = [0; 4];
        numbers[0] = number;
        u256(numbers)
    }
}

impl From<i64> for u256 {
    fn from(number: i64) -> Self {
        match number >= 0 {
            true => From::from(number as u64),
            false => {
                panic!("can't convert negative number");
            }
        }
    }
}
