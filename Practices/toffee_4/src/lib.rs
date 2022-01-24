#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armstrong_numbers_test() {
        let result = are_you_armstrong_number(154);
        assert_eq!(result, false);
    }
}

pub fn are_you_armstrong_number(number: u8) -> bool {
    false
}
