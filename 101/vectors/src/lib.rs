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
}
