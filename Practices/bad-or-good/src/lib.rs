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
    fn calculate_coin_change_in_green_text() {
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
}
