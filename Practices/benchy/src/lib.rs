#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn factorial_of_four_is_complete_test() {
        let result = factorial(4);
        assert_eq!(result, 24);
    }
}

// Sadece benchmark testinde kullanım için faktöryel hesabı ele alındı.
pub fn factorial(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => factorial(num - 1) * num,
    }
}