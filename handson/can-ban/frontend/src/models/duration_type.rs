use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub enum DurationType {
    Hour,
    Day,
    Week,
    Month,
    Unknown,
}

impl FromStr for DurationType {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = match value {
            "Hour" => Self::Hour,
            "Day" => Self::Day,
            "Week" => Self::Week,
            "Month" => Self::Month,
            _ => Self::Day,
        };
        Ok(value)
    }
}
