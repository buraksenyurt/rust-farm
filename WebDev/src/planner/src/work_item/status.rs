use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// Görev durum bilgisini tutan enum sabiti
#[derive(Debug, PartialEq, Deserialize, Clone)]
pub enum Status {
    Ready,
    Doing,
    Completed,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ready => write!(f, "Ready"),
            Status::Doing => write!(f, "Doing"),
            Status::Completed => write!(f, "Completed"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Ready" => Ok(Status::Ready),
            "Doing" => Ok(Status::Doing),
            "Completed" => Ok(Status::Completed),
            _ => Err("Uygun statü bulunamadı".to_string()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn should_parse_for_valid_tshirt_size() {
        assert_eq!(Status::from_str("Ready").unwrap(), Status::Ready);
        assert_eq!(Status::from_str("Doing").unwrap(), Status::Doing);
        assert_eq!(Status::from_str("Completed").unwrap(), Status::Completed);
    }

    #[test]
    #[should_panic]
    fn should_not_parse_for_invalid_tshirt_size() {
        Status::from_str("none").unwrap();
    }
}
