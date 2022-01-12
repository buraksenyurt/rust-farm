#[cfg(test)]
mod tests {
    #[test]
    fn some_tests_about_arrays() {
        let numbers = [1, 3, 4, -9, -18]; // En basit haliyle dizi tanımı
        let numbers_count = numbers.len();
        assert_eq!(numbers_count, 5);
        assert_eq!(numbers[2], 4);

        let numbers: [f32; 4] = [0.1, 2.3, 0.4, 2.6]; // tür belirtilerek dizi tanımı
        assert_eq!(numbers.len(), 4);
        let total: f32 = numbers.into_iter().sum();
        assert_eq!(total, 5.3999996);
    }
}
