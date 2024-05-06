use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WorkItemManager {}

#[wasm_bindgen]
impl WorkItemManager {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn create(
        &self,
        title: String,
        duration: u32,
        duration_type: &str,
        size: &str,
    ) -> Result<bool, JsValue> {
        let work_item = WorkItem {
            id: 1,
            title,
            duration: Some(duration),
            duration_type: Some(DurationType::from_str(duration_type).unwrap()),
            size: Some(Size::from_str(size).unwrap()),
            status: Status::Todo,
        };

        let client = Client::new();
        let res = client
            .post("https://localhost:4448/api/items")
            .json(&work_item)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(true)
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WorkItem {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}

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

#[derive(Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
    Unknown,
}

impl FromStr for Size {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = match value {
            "Small" => Self::Small,
            "Medium" => Self::Medium,
            "Large" => Self::Large,
            "Epic" => Self::Epic,
            _ => Self::Unknown,
        };
        Ok(value)
    }
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Todo,
    Inprogress,
    Completed,
}
