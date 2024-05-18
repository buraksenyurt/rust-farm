use crate::models::*;
use reqwest::Client;
use shared::*;
use std::num::ParseIntError;
use std::str::FromStr;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::js_sys::JsString;

const API_ROOT: &str = "https://localhost:4448/api";
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
            .post(format!("{API_ROOT}/items"))
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
            .patch(format!("{API_ROOT}/items"))
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
            .put(format!("{API_ROOT}/items"))
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
            .get(format!("{API_ROOT}/items"))
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

    pub async fn get_board_report(&self) -> Result<JsString, JsValue> {
        let client = Client::new();
        let res = client
            .get(format!("{API_ROOT}/items/report/summary"))
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
            .get(format!("{API_ROOT}/items/stats/count"))
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
