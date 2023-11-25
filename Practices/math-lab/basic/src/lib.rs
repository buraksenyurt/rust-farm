pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn sub(left: usize, right: usize) -> usize {
    left - right
}
pub fn mul(left: usize, right: usize) -> usize {
    left * right
}
pub fn div(left: usize, right: usize) -> Option<usize> {
    if right == 0 {
        None
    } else {
        Some(left / right)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn sub_works() {
        let result = sub(4, 2);
        assert_eq!(result, 2);
    }

    #[test]
    fn mul_works() {
        let result = mul(3, 2);
        assert_eq!(result, 6);
    }

    #[test]
    fn div_by_zero_works() {
        let result = div(4, 0);
        assert_eq!(result, None);
    }

    #[test]
    fn div_works() {
        let result = div(5, 2);
        assert_eq!(result, Some(2.5 as usize));
    }
}
