use num_bigint::BigInt;
use num_traits::One;
use std::ops::MulAssign;

pub fn factorial(number: u64) -> u64 {
    if number <= 1 {
        1
    } else {
        number * factorial(number - 1)
    }
}

pub fn factorial_for_big(number: u64) -> BigInt {
    let mut result = BigInt::one();
    for i in 1..=number {
        result.mul_assign(&BigInt::from(i));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    #[test]
    fn factorial_of_0_test() {
        assert_eq!(factorial(0), 1);
    }
    #[test]
    fn factorial_of_1_test() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_4_test() {
        assert_eq!(factorial(4), 24);
    }
    #[test]
    fn factorial_of_32_test() {
        let expected = BigInt::from_str("263130836933693530167218012160000000").unwrap();
        assert_eq!(factorial_for_big(32), expected);
    }
}
