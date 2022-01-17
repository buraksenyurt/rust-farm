use std::fmt::{Display, Formatter};

#[cfg(test)]
mod tests {
    use crate::{Colors, Complex, Region, Transparent};

    #[test]
    fn generic_region_type_tests() {
        let golden_horn = Region::<f32> {
            x: 3.14,
            y: -0.0003,
            z: 10.999,
        };
        assert_eq!(golden_horn.z, 10.999);

        let deep_space = Region::<u8>::new(3, 5, 8);
        assert_eq!(deep_space.x, 3);

        let somewhere: Region<String> = Region::<String> {
            x: String::from("Kuzey"),
            y: String::from("Batı"),
            z: String::from("Magma"),
        };
        assert_eq!(somewhere.z, "Magma");
    }

    #[test]
    fn generic_complex_numbers_display_test() {
        let complex1 = Complex {
            real_value: -4,
            virtual_value: 3.2345,
        };
        assert_eq!(complex1.to_string(), "-4+(3.2345)i");
    }

    #[test]
    fn generic_enum_type_test() {
        let green = Colors::Green("#F002023");
        assert_eq!(format!("{:?}", green), "Green(\"#F002023\")");

        let light_blue = Colors::Blue(173216230);
        assert_eq!(format!("{:?}", light_blue), "Blue(173216230)");

        let red_bg = Colors::Red(Transparent { percentage: 0.32 });
        assert_eq!(
            format!("{:?}", red_bg),
            "Red(Transparent { percentage: 0.32 })"
        );
    }
}

// T türüyle çalışan bir struct. x,y,z alanları T türüne bürünür.
pub struct Region<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Region<T> {
    // generic fonksiyon uygulanması
    pub fn new(x: T, y: T, z: T) -> Self {
        Region { x, y, z }
    }
}

// Klasik kobay örneğimiz kompleks sayıların generic versiyonu
struct Complex<T, V> {
    real_value: T,
    virtual_value: V,
}
// Display trait'inin generic Complex türüne uygulanması
// T ve V türlerinin de Display trait'ini uygulamış tipler olması gerekir
// Bunu belirtmezsek `T` doesn't implement `std::fmt::Display` hatasını alırız.
impl<T: Display, V: Display> Display for Complex<T, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+({})i", self.real_value, self.virtual_value)
    }
}

// Bir enum türünü de generic tanımlayabiliriz
#[derive(Debug)]
pub enum Colors<T> {
    Red(T),
    Blue(T),
    Green(T),
}

#[derive(Debug)]
struct Transparent {
    percentage: f32,
}
