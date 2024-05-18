use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub enum Status {
    Todo,
    Inprogress,
    Completed,
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
