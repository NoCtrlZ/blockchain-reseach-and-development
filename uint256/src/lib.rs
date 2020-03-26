mod uint256;
use std::any::type_name;
use crate::uint256::u256;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uint64_to_uint256() {
        let uint64: u64 = 123456789;
        let uint256: u256 = uint64.into();
        assert_eq!(type_of(u256), type_of(uint256));
    }

    #[test]
    fn test_int64_to_uint256() {
        let int64: i64 = 123456789;
        let uint256: u256 = int64.into();
        assert_eq!(type_of(u256), type_of(uint256));
    }

    #[test]
    #[should_panic]
    fn test_negative_int64_convert() {
        let int64: i64 = -123456789;
        let uint256: u256 = int64.into();
    }
}

fn type_of<T>(_: T) -> String {
    type_name::<T>().to_string()
}
