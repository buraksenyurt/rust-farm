use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use shared::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkItemResponse {
    pub id: u32,
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub finish_date: Option<DateTime<Local>>,
    pub size: Option<Size>,
    pub status: Status,
}
