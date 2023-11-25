pub mod collatz;
pub mod fun;

pub fn fibonacci(index: u32) -> u32 {
    if index == 0 {
        return 0;
    } else if index == 1 {
        return 1;
    }

    fibonacci(index - 1) + fibonacci(index - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_works() {
        let result = fibonacci(8);
        assert_eq!(result, 21);
    }
}
