#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_dispatch_test() {
        let hash = produce_hash(Car {
            id: 1,
            price: 300_000.99,
        });
        assert_eq!(hash, "1234");

        let hash = produce_hash(String::from("Bugatti Veyron"));
        assert_eq!(hash, "5678");
    }
}

// Aşağıdaki gibi kobay bir trait ele alalım.
// Güya uygulandığı nesneler için hashcode üretme davranışını ifade etsin.
pub trait Stump {
    fn get_hash(&self) -> String;
}

pub struct Car {
    pub id: i32,
    pub price: f32,
}

// Stump trait'ini kendi veri yapımız ve String türü için uyarladığımızı düşünelim.
impl Stump for Car {
    fn get_hash(&self) -> String {
        String::from("1234")
    }
}

impl Stump for String {
    fn get_hash(&self) -> String {
        String::from("5678")
    }
}

// Klasik olarak kullandığımız yol.
// Fonksiyon generic bir tür belirtiyor ve bunun Stump trait'ini uyarlamış olması gerekiyor.
// Rust burada monomorphization tekniğini kullanarak static dispatch uyguluyor.
// Car ve String için aşağıdaki fonksiyonun birer fonksiyonu hazırlanıyor.
pub fn produce_hash<T: Stump>(object: T) -> String {
    object.get_hash()
}
