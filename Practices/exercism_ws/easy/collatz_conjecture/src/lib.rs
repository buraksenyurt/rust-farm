pub fn collatz(n: u64) -> Option<u64> {
    find_collatz(n, 0)
}
pub fn find_collatz(n: u64, counter: u64) -> Option<u64> {
    match (n, n % 2) {
        (0, _) => None,
        (1, _) => Some(counter),
        (i, 0) => find_collatz(i / 2, counter + 1),
        (i, _) => match i.checked_mul(3)?.checked_add(1) {
            Some(new_i) => find_collatz(new_i, counter + 1),
            None => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one() {
        assert_eq!(Some(0), collatz(1));
    }

    #[test]
    fn sixteen() {
        assert_eq!(Some(4), collatz(16));
    }

    #[test]
    fn twelve() {
        assert_eq!(Some(9), collatz(12));
    }

    #[test]
    fn one_million() {
        assert_eq!(Some(152), collatz(1_000_000));
    }

    #[test]
    fn zero() {
        assert_eq!(None, collatz(0));
    }

    #[test]
    fn test_110243094271() {
        let val = 110243094271;
        assert_eq!(None, collatz(val));
    }

    #[test]
    fn max_div_3() {
        let max = u64::MAX / 3;
        assert_eq!(None, collatz(max));
    }

    #[test]
    fn max_minus_1() {
        let max = u64::MAX - 1;
        assert_eq!(None, collatz(max));
    }
}
