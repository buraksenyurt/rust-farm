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
        let mut arni = Employee::new(
            1,
            String::from("arnıld şıvatzenegır"),
            68,
            String::from("Soni"),
        );
        assert_eq!(arni.level, 68);
        let new_level = arni.change_level(77);
        assert_eq!(new_level, Some(77));

        let new_level = arni.change_level(101);
        assert_eq!(arni.level, 77);
        assert_eq!(new_level, None);
    }

    #[test]
    fn display_behavior_test() {
        let lovri = Employee::new(
            1,
            String::from("cenifır lovrıns"),
            75,
            String::from("Netfiliks"),
        );
        // Display trait'i uyarladığımızdan to_string fonksiyonu bizim istediğimiz şekilde çalışır.
        let lovri_info = lovri.to_string();
        assert_eq!(lovri_info, "1,cenifır lovrıns,75,Netfiliks");
    }

    #[test]
    fn create_vec_of_employees() {
        let mut employees: Vec<Employee> = Vec::new();
        employees.push(Employee::new(
            1,
            String::from("cenifır lovrıns"),
            75,
            String::from("Netfiliks"),
        ));
        employees.push(Employee::new(
            2,
            String::from("arnıld şıvatzenegır"),
            68,
            String::from("Soni"),
        ));
        employees.push(Employee::new(
            3,
            String::from("şan kanıri"),
            100,
            String::from("MGM"),
        ));

        assert_eq!(employees.len(), 3);
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
    // fonksiyon geriye Option enum'ı döner.
    pub fn change_level(&mut self, new_level: u8) -> Option<u8> {
        // yeni değer kontrolü için matching kullandık.
        // 0..100 arası ise sıkıntı yok
        match new_level {
            0..=100 => {
                self.level = new_level;
                Some(self.level)
            }
            _ => None, // Diğer hallerde None
        }
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
