use serde::Serialize;
use std::fmt::{Display, Formatter};

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
