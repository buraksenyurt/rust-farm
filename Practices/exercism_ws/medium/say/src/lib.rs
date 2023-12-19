use std::collections::HashMap;

pub fn encode(n: u64) -> String {
    let mut numbers: HashMap<u64, String> = HashMap::new();
    numbers.insert(0, "zero".to_string());
    numbers.insert(1, "one".to_string());
    numbers.insert(2, "two".to_string());
    numbers.insert(3, "three".to_string());
    numbers.insert(4, "four".to_string());
    numbers.insert(5, "five".to_string());
    numbers.insert(6, "six".to_string());
    numbers.insert(7, "seven".to_string());
    numbers.insert(8, "eight".to_string());
    numbers.insert(9, "nine".to_string());
    numbers.insert(10, "ten".to_string());
    numbers.insert(11, "eleven".to_string());
    numbers.insert(12, "twelve".to_string());
    numbers.insert(13, "thirteen".to_string());
    numbers.insert(14, "fourteen".to_string());
    numbers.insert(15, "fifteen".to_string());
    numbers.insert(16, "sixteen".to_string());
    numbers.insert(17, "seventeen".to_string());
    numbers.insert(18, "eighteen".to_string());
    numbers.insert(19, "nineteen".to_string());
    numbers.insert(20, "twenty".to_string());
    numbers.insert(30, "thirty".to_string());
    numbers.insert(40, "forty".to_string());
    numbers.insert(50, "fifty".to_string());
    numbers.insert(60, "sixty".to_string());
    numbers.insert(70, "seventy".to_string());
    numbers.insert(80, "eighty".to_string());
    numbers.insert(90, "ninety".to_string());
    numbers.insert(1_000, "one thousand".to_string());
    numbers.insert(1_000_000, "one million".to_string());
    numbers.insert(1_000_000_000, "one billion".to_string());

    if let Some(value) = numbers.get(&n) {
        return value.to_string();
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let input = 0;
        let output = encode(input);
        let expected = "zero";
        assert_eq!(output, expected);
    }

    #[test]
    fn one() {
        let input = 1;
        let output = encode(input);
        let expected = "one";
        assert_eq!(output, expected);
    }

    #[test]
    fn fourteen() {
        let input = 14;
        let output = encode(input);
        let expected = "fourteen";
        assert_eq!(output, expected);
    }

    #[test]
    fn twenty() {
        let input = 20;
        let output = encode(input);
        let expected = "twenty";
        assert_eq!(output, expected);
    }

    #[test]
    fn twenty_two() {
        let input = 22;
        let output = encode(input);
        let expected = "twenty-two";
        assert_eq!(output, expected);
    }

    #[test]
    fn thirty() {
        let input = 30;
        let output = encode(input);
        let expected = "thirty";
        assert_eq!(output, expected);
    }

    #[test]
    fn ninety_nine() {
        let input = 99;
        let output = encode(input);
        let expected = "ninety-nine";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_hundred() {
        let input = 100;
        let output = encode(input);
        let expected = "one hundred";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_hundred_twenty_three() {
        let input = 123;
        let output = encode(input);
        let expected = "one hundred twenty-three";
        assert_eq!(output, expected);
    }

    #[test]
    fn two_hundred() {
        let input = 200;
        let output = encode(input);
        let expected = "two hundred";
        assert_eq!(output, expected);
    }

    #[test]
    fn nine_hundred_ninety_nine() {
        let input = 999;
        let output = encode(input);
        let expected = "nine hundred ninety-nine";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_thousand() {
        let input = 1000;
        let output = encode(input);
        let expected = "one thousand";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_thousand_two_hundred_thirty_four() {
        let input = 1234;
        let output = encode(input);
        let expected = "one thousand two hundred thirty-four";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_million() {
        let input = 1000000;
        let output = encode(input);
        let expected = "one million";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_million_two_thousand_three_hundred_forty_five() {
        let input = 1002345;
        let output = encode(input);
        let expected = "one million two thousand three hundred forty-five";
        assert_eq!(output, expected);
    }

    #[test]
    fn one_billion() {
        let input = 1000000000;
        let output = encode(input);
        let expected = "one billion";
        assert_eq!(output, expected);
    }

    #[test]
    fn a_big_number() {
        let input = 987654321123;
        let output = encode(input);
        let expected = "nine hundred eighty-seven billion six hundred fifty-four million three hundred twenty-one thousand one hundred twenty-three";
        assert_eq!(output, expected);
    }

    #[test]
    fn max_i64() {
        let input = 9223372036854775807;
        let output = encode(input);
        let expected = "nine quintillion two hundred twenty-three quadrillion three hundred seventy-two trillion thirty-six billion eight hundred fifty-four million seven hundred seventy-five thousand eight hundred seven";
        assert_eq!(output, expected);
    }

    #[test]
    fn max_u64() {
        let input = 18446744073709551615;
        let output = encode(input);
        let expected = "eighteen quintillion four hundred forty-six quadrillion seven hundred forty-four trillion seventy-three billion seven hundred nine million five hundred fifty-one thousand six hundred fifteen";
        assert_eq!(output, expected);
    }
}
