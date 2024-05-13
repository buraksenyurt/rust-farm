use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::num::ParseIntError;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::JsString;

#[wasm_bindgen]
pub struct WorkItemManager {}

impl Default for WorkItemManager {
    fn default() -> Self {
        Self::new()
    }
}

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
    ) -> Result<JsString, JsValue> {
        let work_item = CreateWorkItemRequest {
            title,
            duration: Some(duration),
            duration_type: Some(DurationType::from_str(duration_type).unwrap()),
            size: Some(Size::from_str(size).unwrap()),
        };

        let client = Client::new();
        let res = client
            .post("http://localhost:4448/api/items")
            .json(&work_item)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if res.status().is_success() {
            let json_response = res
                .text()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsString::from(json_response))
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }

    pub async fn move_to_archive(&self, id: u32) -> Result<(), JsValue> {
        let request = MoveToArchiveRequest { id };
        let client = Client::new();
        let res = client
            .patch("http://localhost:4448/api/items")
            .json(&request)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }

    pub async fn change_status(&self, id: u32, status: &str) -> Result<(), JsValue> {
        let update_item = UpdateStatusRequest {
            id,
            new_status: Status::from_str(status).unwrap(),
        };

        let client = Client::new();
        let res = client
            .put("http://localhost:4448/api/items")
            .json(&update_item)
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }

    pub async fn get_board(&self) -> Result<JsString, JsValue> {
        let client = Client::new();
        let res = client
            .get("http://localhost:4448/api/items")
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            let json_response = res
                .text()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsString::from(json_response))
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }

    pub async fn get_items_count(&self) -> Result<u32, JsValue> {
        let client = Client::new();
        let res = client
            .get("http://localhost:4448/api/items/stats/count")
            .send()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        if res.status().is_success() {
            let count_string = res
                .text()
                .await
                .map_err(|e| JsValue::from_str(&e.to_string()))?;

            let count: u32 = count_string
                .trim()
                .parse()
                .map_err(|e: ParseIntError| JsValue::from_str(&e.to_string()))?;

            Ok(count)
        } else {
            Err(JsValue::from_str(&format!("Send error: {}", res.status())))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateWorkItemRequest {
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateWorkItemResponse {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub id: u32,
    pub new_status: Status,
}

#[derive(Serialize, Deserialize)]
pub struct MoveToArchiveRequest {
    pub id: u32,
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
