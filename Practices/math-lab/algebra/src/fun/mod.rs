pub fn which_fizz_buzz(num: u32) -> String {
    return if num % 3 == 0 && num % 5 == 0 {
        String::from("FizzBuzz")
    } else if num % 3 == 0 {
        String::from("Fizz")
    } else if num % 5 == 0 {
        String::from("Buzz")
    } else {
        num.to_string()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_15_is_buzz_works() {
        let result = which_fizz_buzz(55);
        assert_eq!(result, "Buzz");
    }
    #[test]
    fn value_27_is_fizz_works() {
        let result = which_fizz_buzz(27);
        assert_eq!(result, "Fizz");
    }
    #[test]
    fn value_30_is_fizz_works() {
        let result = which_fizz_buzz(30);
        assert_eq!(result, "FizzBuzz");
    }
}
