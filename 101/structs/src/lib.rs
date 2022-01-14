use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Bir çalışanın birkaç bilgisini tutacak türden bir veri yapısı tanımladık
pub struct Employee {
    id: i32,
    name: String,
    level: u8,
    company: String,
}

// Veri yapımız için birkaç fonksiyon bildirimi
impl Employee {
    // Yeni bir employee nesnesi oluşturmak için kullanabiliriz.
    fn new(id: i32, name: String, level: u8, company: String) -> Employee {
        // argüman adları Employee struct'ınınkilerle aynı olduğundan
        // id:e_id gibi bir atama yapmaya gerek kalmaz
        Employee {
            id,
            name,
            level,
            company,
        }
    }

    // O nesnenin level değerini değiştiren örnek fonksiyon
    // self ile çalışma zamanı nesnesinin kendisi ifade edilir.
    fn change_level(&mut self, new_level: u8) {
        self.level = new_level;
    }
}

// Daha çok meta programming'de öne çıkan ama nesnelere soyut davranışları entegre etmek için de
// kullanılan Trait'ler için örnek implementasyon.
// Sistemin varsayılan Display davranışını Employee veri yapısı için yeniden yazıyoruz
impl Display for Employee {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.id, self.name, self.level, self.company
        )
    }
}
