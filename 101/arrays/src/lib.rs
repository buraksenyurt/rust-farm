#[cfg(test)]
mod tests {
    use crate::get_levels;

    #[test]
    fn length_test() {
        let numbers = [1, 3, 4, -9, -18]; // En basit haliyle dizi tanımı
        let numbers_count = numbers.len();
        assert_eq!(numbers_count, 5);
        assert_eq!(numbers[2], 4);
    }

    #[test]
    fn total_value_of_array_test() {
        let numbers: [f32; 4] = [0.1, 2.3, 0.4, 2.6]; // tür belirtilerek dizi tanımı
        let total: f32 = numbers.into_iter().sum();
        assert_eq!(total, 5.3999996);
    }

    #[test]
    fn init_same_content_test() {
        let numbers = [1; 10]; // 10 tane 1 rakamından oluşan dizi tanımı
        let result = numbers.into_iter().any(|n| n != 1);
        assert_eq!(result, false);
    }

    #[test]
    fn fold_func_test() {
        let numbers = [5, 6, 7, 8, 9];
        let result = numbers.iter().fold(0, |total, next| total + next);
        assert_eq!(result, 35);
    }

    #[test]
    fn mutability_test() {
        let mut numbers = [1, 3, 5, 7];
        numbers[3] += 1;
        assert_eq!(numbers[3], 8);
    }

    #[test]
    fn get_array_from_fn_test() {
        let levels = get_levels();
        assert_eq!(levels[1], 200);
    }
}

pub fn get_levels() -> [i32; 4] {
    [100, 200, 300, 400]
}
