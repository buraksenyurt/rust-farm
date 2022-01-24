#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn armstrong_numbers_test() {
        assert_eq!(are_you_armstrong_number(9), true);
        assert_eq!(are_you_armstrong_number(10), false);
        assert_eq!(are_you_armstrong_number(153), true);
        assert_eq!(are_you_armstrong_number(154), false);
        assert_eq!(are_you_armstrong_number(371), true);
        assert_eq!(are_you_armstrong_number(999), false);
    }
}

pub fn are_you_armstrong_number(number: u32) -> bool {
    // İlk olarak sayının rakamlarını çekmek isterim.
    // 10 düzende to_digit fonksiyonu iş görecektir
    let digits: Vec<_> = number
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let digits_count = digits.len() as u32;
    let sum_of: u32 = digits.iter().map(|n| n.pow(digits_count)).sum();
    sum_of == number
}
