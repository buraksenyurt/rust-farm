use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkItem {
    pub id: u32,
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
    pub status: Status,
    pub crate_date: DateTime<Local>,
    pub modified_date: Option<DateTime<Local>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWorkItemRequest {
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkItemResponse {
    pub id: u32,
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateStatusRequest {
    pub id: u32,
    pub new_status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveToArchiveRequest {
    pub id: u32,
}

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
