#[cfg(test)]
mod tests {
    use crate::Average;

    #[test]
    fn odd_numbers_average_test() {
        let numbers = vec![3, 2, 6, 9, 1, 0, -5, 15];
        // Görüldüğü üzere vector üstünde kendi fonksiyonumuzu kullanabiliyoruz.
        let result = numbers.avg_for_odds();
        assert_eq!(result, 4);
    }
}

// Aşağıdaki generic trait'i ele alalım.
// avg_for_odds isimli bir fonksiyon bildirimi var.
// Uygulandığı tür üstünde bir şeyler yapıp geriye T türünden değer döndürecek
trait Average<T> {
    fn avg_for_odds(&self) -> T;
}

// Bu trait'i i32 türünden bir Vector'e uygulayabiliriz.
// Sadece vektör sayılarında 3 ile bölünebilenlerin ortalamasını alıyor.
impl Average<i32> for Vec<i32> {
    fn avg_for_odds(&self) -> i32 {
        let mut total = 0;
        for n in self {
            if n % 3 == 0 {
                total += n;
            }
        }
        total / (self.len() as i32)
    }
}
