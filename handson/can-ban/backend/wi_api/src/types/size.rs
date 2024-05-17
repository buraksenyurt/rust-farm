use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Size {
    Small = 1,
    Medium = 2,
    Large = 3,
    Epic = 4,
    Unknown = 0,
}

impl TryFrom<u8> for Size {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Self::Small as u8 => Ok(Self::Small),
            v if v == Self::Medium as u8 => Ok(Self::Medium),
            v if v == Self::Large as u8 => Ok(Self::Large),
            v if v == Self::Epic as u8 => Ok(Self::Epic),
            v if v == Self::Unknown as u8 => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}
