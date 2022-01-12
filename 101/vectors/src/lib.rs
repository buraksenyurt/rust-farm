#[cfg(test)]
mod tests {
    #[test]
    fn create_vector_with_new_fn_test() {
        // new fonksiyonu ile f32 türünden elemanlar taşıyacak mutable bir vector tanımlanır
        let mut points: Vec<f32> = Vec::new();
        // push fonksiyonu ile vector nesnesine yeni elemanlar eklenebilir
        points.push(49.99);
        points.push(89.50);
        points.push(100.00);
        assert_eq!(points.len(), 3);
        // pop ile son eklenen eleman listeden çıkarılarak elde edilir
        assert_eq!(points.pop(), Some(100.00));
        // Bu nedenle eleman sayısı 1 azalır
        assert_eq!(points.len(), 2);
    }

    #[test]
    fn create_vector_with_macro_test() {
        // Bir vector pratik olarak vec! makrosu ile de oluşturulabilir
        let mut numbers = vec![5, 3, 9, 2, -12, -14, 7];
        assert_eq!(numbers.len(), 7);
        numbers.remove(3);
        numbers.remove(0);
        assert_eq!(numbers, [3, 9, -12, -14, 7]);
        assert_eq!(numbers.len(), 5);
    }

    #[test]
    fn change_vector_elements_test() {
        // 10 tane 1 rakamından oluşan bir vector tanımı
        let mut numbers = vec![1; 5];
        // for_each ile var olan vector içindeki elemanları değiştiriyoruz.
        numbers.iter_mut().for_each(|n| *n += 1);
        assert_eq!(numbers[2], 2);
    }
}
