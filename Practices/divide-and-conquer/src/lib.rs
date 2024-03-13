pub fn get_max_value(values: &[i32]) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_max_value_returns_valid_value_test() {
        let numbers = vec![5, 6, 1, 9, 10, 2, 8, 2, 7, 7, 10, 32, 23, 0, 6, 21, 22];
        let actual = get_max_value(&numbers);
        let expected = 32;
        assert_eq!(actual, expected);
    }
}
