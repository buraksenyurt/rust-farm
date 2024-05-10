use serde::{Deserialize, Serialize};
//use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkItem {
    pub id: u32,
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum DurationType {
    Hour,
    Day,
    Week,
    Month,
    Unknown,
}

// impl FromStr for DurationType {
//     type Err = ();

//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         match value {
//             "Hour" => Ok(Self::Hour),
//             "Day" => Ok(Self::Day),
//             "Week" => Ok(Self::Week),
//             "Month" => Ok(Self::Month),
//             _ => Ok(Self::Unknown),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
    Unknown,
}

// impl FromStr for Size {
//     type Err = ();

//     fn from_str(value: &str) -> Result<Self, Self::Err> {
//         match value {
//             "Small" => Ok(Self::Small),
//             "Medium" => Ok(Self::Medium),
//             "Large" => Ok(Self::Large),
//             "Epic" => Ok(Self::Epic),
//             _ => Ok(Self::Unknown),
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Todo,
    Inprogress,
    Completed,
}
