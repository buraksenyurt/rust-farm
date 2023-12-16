pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => {
            let mut current_number = 2;
            let mut counter = 0;
            loop {
                current_number += 1;
                if is_prime_number(current_number) {
                    counter += 1;
                }
                if counter == n {
                    break;
                }
            }
            current_number
        }
    }
}

pub fn is_prime_number(number: u32) -> bool {
    let mut i = 2;
    while i * i <= number {
        if number % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_prime() {
        assert_eq!(nth(0), 2);
    }

    #[test]
    fn second_prime() {
        assert_eq!(nth(1), 3);
    }

    #[test]
    fn sixth_prime() {
        assert_eq!(nth(5), 13);
    }

    #[test]
    fn big_prime() {
        assert_eq!(nth(10_000), 104_743);
    }
}
