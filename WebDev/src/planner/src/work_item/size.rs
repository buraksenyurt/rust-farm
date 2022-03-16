use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Debug, PartialEq)]
pub enum Size {
    Short = 1,
    Medium = 3,
    Large = 5,
    Xlarge = 8,
    Epic = 13,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Short => write!(f, "Ufak"),
            Size::Medium => write!(f, "Orta"),
            Size::Large => write!(f, "İri"),
            Size::Xlarge => write!(f, "Koccaman"),
            Size::Epic => write!(f, "İnanılmaz"),
        }
    }
}

impl FromStr for Size {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "1" => Ok(Size::Short),
            "3" => Ok(Size::Medium),
            "5" => Ok(Size::Large),
            "8" => Ok(Size::Xlarge),
            "13" => Ok(Size::Epic),
            _ => Err("Uygun T-Shirt Size bulunamadı".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_parse_for_valid_tshirt_size() {
        assert_eq!(Size::from_str("1").unwrap(), Size::Short);
        assert_eq!(Size::from_str("3").unwrap(), Size::Medium);
        assert_eq!(Size::from_str("5").unwrap(), Size::Large);
        assert_eq!(Size::from_str("8").unwrap(), Size::Xlarge);
        assert_eq!(Size::from_str("13").unwrap(), Size::Epic);
    }

    #[test]
    #[should_panic]
    fn should_not_parse_for_invalid_tshirt_size() {
        Size::from_str("20").unwrap();
    }
}
