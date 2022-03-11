use std::fmt::{Display, Formatter};

/// Bir iş için ortak olan alanları tutar
/// İşin başlığı, değeri ve durumu taşınır.
#[derive(Debug, PartialEq)]
pub struct Base {
    pub title: String,
    pub value: u16,
    pub status: String,
}

impl Base {
    pub fn new(input_title: &str, input_value: u16, input_status: &str) -> Self {
        Base {
            title: input_title.to_string(),
            value: input_value,
            status: input_status.to_string(),
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.({}:{})", self.title, self.status, self.value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_works() {
        let item = Base::new("Odayı temizle", 5, "Ready");
        assert_eq!(item.to_string(), "Odayı temizle.(Ready:5)");
    }
}
