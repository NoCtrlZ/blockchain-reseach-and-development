mod uint256;
use crate::uint256::u256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let uint256 = u64_to_u256();
        println!("{:?}", uint256);
    }
}

fn u64_to_u256() -> u256 {
    let uint64: u64 = 123456789;
    uint64.into()
}
