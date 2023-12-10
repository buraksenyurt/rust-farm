pub fn is_armstrong_number(num: u32) -> bool {
    if num <= 9 {
        return true;
    }
    let digits_count = digits(num as usize).count();
    let mut sum: usize = 0;
    for d in digits(num as usize) {
        sum += d.pow(digits_count as u32);
    }
    sum == num as usize
}
fn digits(mut num: usize) -> impl Iterator<Item = usize> {
    let mut divisor = 1;
    while num >= divisor * 10 {
        divisor *= 10;
    }
    std::iter::from_fn(move || {
        if divisor == 0 {
            None
        } else {
            let v = num / divisor;
            num %= divisor;
            divisor /= 10;
            Some(v)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    fn there_are_no_2_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    fn three_digit_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn three_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    fn four_digit_armstrong_number() {
        assert!(is_armstrong_number(9474))
    }

    #[test]
    fn four_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9475))
    }

    #[test]
    fn seven_digit_armstrong_number() {
        assert!(is_armstrong_number(9_926_315))
    }

    #[test]
    fn seven_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(9_926_316))
    }

    #[test]
    fn nine_digit_armstrong_number() {
        assert!(is_armstrong_number(912_985_153));
    }

    #[test]
    fn nine_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(999_999_999));
    }

    #[test]
    fn ten_digit_non_armstrong_number() {
        assert!(!is_armstrong_number(3_999_999_999));
    }

    // The following number has an Armstrong sum equal to 2^32 plus itself,
    // and therefore will be detected as an Armstrong number if you are
    // incorrectly using wrapping arithmetic.
    #[test]
    fn properly_handles_overflow() {
        assert!(!is_armstrong_number(4_106_098_957));
    }
}
