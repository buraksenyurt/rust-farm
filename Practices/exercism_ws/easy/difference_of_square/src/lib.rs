pub fn square_of_sum(n: u32) -> u32 {
    todo!("square of sum of 1...{n}")
}

pub fn sum_of_squares(n: u32) -> u32 {
    todo!("sum of squares of 1...{n}")
}

pub fn difference(n: u32) -> u32 {
    todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_of_sum_1() {
        assert_eq!(1, square_of_sum(1));
    }

    #[test]
    fn square_of_sum_5() {
        assert_eq!(225, square_of_sum(5));
    }

    #[test]
    fn square_of_sum_100() {
        assert_eq!(25_502_500, square_of_sum(100));
    }

    #[test]
    fn sum_of_squares_1() {
        assert_eq!(1, sum_of_squares(1));
    }

    #[test]
    fn sum_of_squares_5() {
        assert_eq!(55, sum_of_squares(5));
    }

    #[test]
    fn sum_of_squares_100() {
        assert_eq!(338_350, sum_of_squares(100));
    }

    #[test]
    fn difference_1() {
        assert_eq!(0, difference(1));
    }

    #[test]
    fn difference_5() {
        assert_eq!(170, difference(5));
    }

    #[test]
    fn difference_100() {
        assert_eq!(25_164_150, difference(100));
    }
}
