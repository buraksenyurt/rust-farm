pub fn factorial(_number: u64) -> u64 {
    unimplemented!()
}
#[cfg(test)]
mod tests {
    use super::*;
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
}