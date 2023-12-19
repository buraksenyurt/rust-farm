const UNITS: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

pub fn encode(n: u64) -> String {
    if n < 20 {
        UNITS[n as usize].to_string()
    } else if n <= 99 {
        format!(
            "{}{}",
            TENS[(n / 10) as usize],
            if n % 10 > 0 {
                format!("-{}", encode(n % 10))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999 {
        format!(
            "{} hundred{}",
            UNITS[(n / 100) as usize],
            if n % 100 > 0 {
                format!(" {}", encode(n % 100))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999_999 {
        format!(
            "{} thousand{}",
            encode(n / 1000),
            if n % 1000 > 0 {
                format!(" {}", encode(n % 1000))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999_999_999 {
        format!(
            "{} million{}",
            encode(n / 1_000_000),
            if n % 1_000_000 > 0 {
                format!(" {}", encode(n % 1_000_000))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999_999_999_999 {
        format!(
            "{} billion{}",
            encode(n / 1_000_000_000),
            if n % 1_000_000_000 > 0 {
                format!(" {}", encode(n % 1_000_000_000))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999_999_999_999_999 {
        format!(
            "{} trillion{}",
            encode(n / 1_000_000_000_000),
            if n % 1_000_000_000_000 > 0 {
                format!(" {}", encode(n % 1_000_000_000_000))
            } else {
                "".to_string()
            }
        )
    } else if n <= 999_999_999_999_999_999 {
        format!(
            "{} quadrillion{}",
            encode(n / 1_000_000_000_000_000),
            if n % 1_000_000_000_000_000 > 0 {
                format!(" {}", encode(n % 1_000_000_000_000_000))
            } else {
                "".to_string()
            }
        )
    } else {
        format!(
            "{} quintillion{}",
            encode(n / 1_000_000_000_000_000_000),
            if n % 1_000_000_000_000_000_000 > 0 {
                format!(" {}", encode(n % 1_000_000_000_000_000_000))
            } else {
                "".to_string()
            }
        )
    }
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
        let input = 1_000_000_000;
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
