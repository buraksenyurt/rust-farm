use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
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
