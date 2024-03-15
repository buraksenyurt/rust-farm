pub struct Fibonacci;

impl Fibonacci {
    pub fn calc_worst(number: u64) -> u64 {
        if number <= 1 {
            return number;
        }
        Self::calc_worst(number - 1) + Self::calc_worst(number - 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fibonacci_in_worst() {
        let actual = Fibonacci::calc_worst(32);
        let expected = 2178309;
        assert_eq!(actual, expected);
    }
}
