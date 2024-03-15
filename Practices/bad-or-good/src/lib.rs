use std::collections::HashMap;

pub struct Fibonacci;

impl Fibonacci {
    pub fn calc_worst(number: u64) -> u64 {
        if number <= 1 {
            return number;
        }
        Self::calc_worst(number - 1) + Self::calc_worst(number - 2)
    }

    pub fn calc_green(number: u64, memoization: &mut HashMap<u64, u64>) -> u64 {
        if number <= 1 {
            return number;
        }
        // Sayı daha önceden hesaplanmışsa HashMap'ten getir
        if memoization.contains_key(&number) {
            return *memoization.get(&number).unwrap();
        }
        // Sayı Hashmap'te yoksa fibonacci toplamını yap
        let result = Self::calc_green(number - 1, memoization) + Self::calc_green(number - 2, memoization);
        // bulunan değeri HashMap'e insert et
        memoization.insert(number, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fibonacci_in_worst() {
        let actual = Fibonacci::calc_worst(32);
        let expected = 2_178_309;
        assert_eq!(actual, expected);
    }

    #[test]
    fn calculate_fibonacci_in_green() {
        let mut memo_set: HashMap<u64, u64> = HashMap::new();
        let actual = Fibonacci::calc_green(51, &mut memo_set);
        let expected = 20_365_011_074;
        assert_eq!(actual, expected);
    }
}
