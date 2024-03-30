use std::collections::{HashMap, VecDeque};

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
        let result =
            Self::calc_green(number - 1, memoization) + Self::calc_green(number - 2, memoization);
        // bulunan değeri HashMap'e insert et
        memoization.insert(number, result);
        result
    }
}

pub struct CoinChange;

impl CoinChange {
    pub fn calc_worst(coins: &[i32], amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }
        let mut res = i32::MAX;
        for &coin in coins {
            if amount - coin >= 0 {
                let sub_res = Self::calc_worst(coins, amount - coin);
                if sub_res != i32::MAX && sub_res + 1 < res {
                    res = sub_res + 1;
                }
            }
        }
        res
    }

    pub fn calc_green(coins: &[i32], amount: i32, memoization: &mut Vec<i32>) -> i32 {
        if amount == 0 {
            return 0;
        }
        if memoization[amount as usize] != -1 {
            return memoization[amount as usize];
        }
        let mut result = i32::MAX;
        for &coin in coins {
            if amount - coin >= 0 {
                let sub_res = Self::calc_green(coins, amount - coin, memoization);
                if sub_res != i32::MAX && sub_res + 1 < result {
                    result = sub_res + 1;
                }
            }
        }
        memoization[amount as usize] = result;
        result
    }
}

pub enum Coin {
    Penny = 1,
    Nickel = 5,
    Dime = 10,
    Quarter = 25,
}

pub struct StringOps;

impl StringOps {
    pub fn calc_worst(words: Vec<String>) -> String {
        let mut output = String::new();
        for word in words {
            output += &word;
        }
        output
    }

    pub fn calc_green(words: Vec<String>) -> String {
        // Önce toplam uzunluğu bulalım.
        let len = words.iter().map(|w| w.len()).sum();
        // Bulunun uzunluk kadar boyuta sahip bir String tanımlayalım
        let mut output = String::with_capacity(len);
        for word in words {
            output.push_str(&word);
        }
        output
    }
}

pub struct Sorting;

impl Sorting {
    pub fn radix(data: &mut Vec<i32>) {
        let max = *data.iter().max().unwrap_or(&0) as usize;
        let mut digit_place = 1;

        while max / digit_place > 0 {
            let mut buckets: Vec<VecDeque<i32>> = vec![VecDeque::new(); 10];

            for &number in data.iter() {
                let digit = (number as usize / digit_place) % 10;
                buckets[digit].push_back(number);
            }

            let mut index = 0;
            for bucket in buckets {
                for &number in bucket.iter() {
                    data[index] = number;
                    index += 1;
                }
            }

            digit_place *= 10;
        }
    }

    pub fn bubble(data: &mut Vec<i32>) {
        let mut n = data.len();
        let mut swapped = true;

        while swapped {
            swapped = false;
            for i in 1..n {
                if data[i - 1] > data[i] {
                    data.swap(i - 1, i);
                    swapped = true;
                }
            }
            n -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_fibonacci_in_worst_test() {
        let actual = Fibonacci::calc_worst(32);
        let expected = 2_178_309;
        assert_eq!(actual, expected);
    }

    #[test]
    fn calculate_fibonacci_in_green_test() {
        let mut memo_set: HashMap<u64, u64> = HashMap::new();
        let actual = Fibonacci::calc_green(51, &mut memo_set);
        let expected = 20_365_011_074;
        assert_eq!(actual, expected);
    }

    #[test]
    fn calculate_coin_change_in_worst_test() {
        /*
           1 Penny = 1 Cent
           1 Nickel = 5 Cents
           1 Dime =  10 Cents
           1 Quarter = 25 Cents
        */
        let coins = vec![
            Coin::Penny as i32,
            Coin::Nickel as i32,
            Coin::Dime as i32,
            Coin::Quarter as i32,
        ];
        let amount = 41;
        let actual = CoinChange::calc_worst(&coins, amount);
        assert_eq!(actual, 4); // 41 = 1 + 5 + 10 + 25 -> Toplam 4 adet bozuk para
    }

    #[test]
    fn calculate_coin_change_in_green_test() {
        let coins = vec![
            Coin::Penny as i32,
            Coin::Nickel as i32,
            Coin::Dime as i32,
            Coin::Quarter as i32,
        ];
        let amount = 41;
        let mut memo_set = vec![-1; (amount + 1) as usize];
        memo_set[0] = 0;
        let result = CoinChange::calc_green(&coins, amount, &mut memo_set);
        assert_eq!(result, 4); // 41 = 1 + 5 + 10 + 25 -> Toplam 4 adet bozuk para
    }

    #[test]
    fn concatenate_string_in_worst_test() {
        let words = vec![
            "C#".to_string(),
            "Python".to_string(),
            "Go".to_string(),
            "Rust".to_string(),
            "C++".to_string(),
            "C".to_string(),
            "Assembly".to_string(),
            "Cobol".to_string(),
            "Delphi".to_string(),
        ];
        let actual = StringOps::calc_worst(words);
        let expected = "C#PythonGoRustC++CAssemblyCobolDelphi".to_string();
        assert_eq!(actual, expected);
    }

    #[test]
    fn concatenate_string_in_green_test() {
        let words = vec![
            "C#".to_string(),
            "Python".to_string(),
            "Go".to_string(),
            "Rust".to_string(),
            "C++".to_string(),
            "C".to_string(),
            "Assembly".to_string(),
            "Cobol".to_string(),
            "Delphi".to_string(),
        ];
        let actual = StringOps::calc_green(words);
        let expected = "C#PythonGoRustC++CAssemblyCobolDelphi".to_string();
        assert_eq!(actual, expected);
    }
}
