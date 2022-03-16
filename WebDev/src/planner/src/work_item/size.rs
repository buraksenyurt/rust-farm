use serde_repr::*;
use std::fmt::{Display, Formatter};

#[derive(Serialize_repr, Debug, PartialEq)]
#[repr(u8)]
pub enum Size {
    Short = 1,
    Medium = 3,
    Large = 5,
    Xlarge = 8,
    Epic = 13,
    Unknown = 0,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Short => write!(f, "Ufak"),
            Size::Medium => write!(f, "Orta"),
            Size::Large => write!(f, "İri"),
            Size::Xlarge => write!(f, "Koccaman"),
            Size::Epic => write!(f, "İnanılmaz"),
            Size::Unknown => write!(f, "Bilinmiyor"),
        }
    }
}

impl From<u64> for Size {
    fn from(v: u64) -> Self {
        match v {
            1 => Size::Short,
            3 => Size::Medium,
            5 => Size::Large,
            8 => Size::Xlarge,
            13 => Size::Epic,
            _ => Size::Unknown,
        }
    }
}
// impl FromStr for Size {
//     type Err = String;
//
//     fn from_str(s: &str) -> Result<Self, String> {
//         match s {
//             "1" => Ok(Size::Short),
//             "3" => Ok(Size::Medium),
//             "5" => Ok(Size::Large),
//             "8" => Ok(Size::Xlarge),
//             "13" => Ok(Size::Epic),
//             _ => Err("Uygun T-Shirt Size bulunamadı".to_string()),
//         }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_for_valid_tshirt_size() {
        assert_eq!(Size::from(1), Size::Short);
        assert_eq!(Size::from(1), Size::Short);
        assert_eq!(Size::from(3), Size::Medium);
        assert_eq!(Size::from(5), Size::Large);
        assert_eq!(Size::from(8), Size::Xlarge);
        assert_eq!(Size::from(13), Size::Epic);
        assert_eq!(Size::from(20), Size::Unknown);
    }

    // #[test]
    // #[should_panic]
    // fn should_not_parse_for_invalid_tshirt_size() {
    //     Size::from(20);
    // }
}
