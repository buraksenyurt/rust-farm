use std::fmt::{Display, Formatter};
use std::ops::Add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_numbers_average_test() {
        let numbers = vec![3, 2, 6, 9, 1, 0, -5, 15];
        // Görüldüğü üzere vector üstünde kendi fonksiyonumuzu kullanabiliyoruz.
        let result = numbers.avg_for_odds();
        assert_eq!(result, 4);
    }

    #[test]
    fn add_operator_overload_test() {
        let east = Location {
            x: 32,
            y: 34,
            z: 10,
        };
        let west = Location {
            x: -10,
            y: -18,
            z: 4,
        };
        let total = east + west;
        assert_eq!(total.to_string(), "(22:16:14)");
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

// Bu sefer kendi veri yapımız için Operator Overloading yapıyoruz.
// Yani Location veri türü için toplama operasyonunu çalışma zamanına yeniden öğretiyoruz.
impl Add for Location {
    // Çıktı olarak Location nesnesi dönüleceği söylenir
    type Output = Location;

    // + operatörü ile karşılaşınca ne yapılacağı bu fonksiyonda kodlanır.
    fn add(self, other: Self) -> Self::Output {
        Location {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

struct Location {
    x: i32,
    y: i32,
    z: i32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.x, self.y, self.z)
    }
}
