use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

impl FromStr for DurationType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Hour" => Ok(Self::Hour),
            "Day" => Ok(Self::Day),
            "Week" => Ok(Self::Week),
            "Month" => Ok(Self::Month),
            _ => Ok(Self::Unknown),
        }
    }
}

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

impl FromStr for Size {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = match value {
            "Small" => Self::Small,
            "Medium" => Self::Medium,
            "Large" => Self::Large,
            "Epic" => Self::Epic,
            _ => Self::Small,
        };
        Ok(value)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Status {
    Todo = 1,
    Inprogress = 2,
    Completed = 3,
}

impl TryFrom<u8> for Status {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            v if v == Self::Todo as u8 => Ok(Self::Todo),
            v if v == Self::Inprogress as u8 => Ok(Self::Inprogress),
            v if v == Self::Completed as u8 => Ok(Self::Completed),
            _ => Err(()),
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = match value {
            "ToDo" => Self::Todo,
            "InProgress" => Self::Inprogress,
            "Completed" => Self::Completed,
            _ => Self::Todo,
        };
        Ok(value)
    }
}
