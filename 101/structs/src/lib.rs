use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use crate::Employee;

    #[test]
    fn create_employee_object_test() {
        let betty = Employee {
            id: 1,
            name: String::from("beti deyvis"),
            level: 90,
            company: String::from("Wornır buros"),
        };
        assert_eq!(betty.level, 90);
        assert_eq!(betty.name, "beti deyvis");

        let sean = Employee::new(2, String::from("şan kanıri"), 100, String::from("MGM"));
        assert_eq!(sean.company, "MGM");
    }

    #[test]
    fn change_employee_level_test() {
        let mut arni = Employee {
            id: 1,
            name: String::from("arnıld şıvatzenegır"),
            level: 68,
            company: String::from("Soni"),
        };
        assert_eq!(arni.level, 68);
        arni.change_level(77);
        assert_eq!(arni.level, 77);
    }

    #[test]
    fn display_behavior_test() {}

    #[test]
    fn create_vec_of_employees() {}
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
    pub fn new(id: i32, name: String, level: u8, company: String) -> Employee {
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
    pub fn change_level(&mut self, new_level: u8) {
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
