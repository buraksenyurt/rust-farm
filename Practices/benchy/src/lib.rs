#[cfg(test)]
mod tests {
    use crate::dummy;

    #[test]
    fn dummy_func_works() {
        let result = dummy(8);
        assert_eq!(result, 64);
    }
}

// Sadece benchmark testinde kullanm için aptalca bir fonksiyon dahil edildi
pub fn dummy(number: u128) -> u128 {
    number * number
}