use crate::Size;
use std::fmt::{Display, Formatter};

/// Bir iş için ortak olan alanları tutar
/// İşin başlığı, değeri ve durumu taşınır.
#[derive(Debug, PartialEq)]
pub struct Base {
    pub title: String,
    pub status: String,
    pub size: Size,
}

impl Base {
    pub fn new(input_title: &str, input_status: &str, input_size: Size) -> Self {
        Base {
            title: input_title.to_string(),
            status: input_status.to_string(),
            size: input_size,
        }
    }
}

impl Display for Base {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.({}-{})", self.title, self.status, self.size)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_new_works() {
        let item = Base::new("Odayı temizle", "Ready", Size::Short);
        assert_eq!(item.to_string(), "Odayı temizle.(Ready-Ufak)");
    }
}
