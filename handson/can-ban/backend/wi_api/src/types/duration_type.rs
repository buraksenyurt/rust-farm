use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DurationType {
    Hour = 1,
    Day = 2,
    Week = 3,
    Month = 4,
    Unknown = 5,
}

impl TryFrom<u8> for DurationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Self::Hour as u8 => Ok(Self::Hour),
            v if v == Self::Day as u8 => Ok(Self::Day),
            v if v == Self::Week as u8 => Ok(Self::Week),
            v if v == Self::Month as u8 => Ok(Self::Month),
            v if v == Self::Unknown as u8 => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}
