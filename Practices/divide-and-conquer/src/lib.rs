pub fn get_max_value(values: &[i32]) -> i32 {
    if values.len() == 1 {
        values[0]
    } else {
        let mid = values.len() / 2;
        let first_max = get_max_value(&values[..mid]);
        let second_max = get_max_value(&values[mid..]);
        if first_max > second_max {
            first_max
        } else {
            second_max
        }
    }
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
