use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use shared::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkItem {
    pub id: u32,
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
    pub status: Status,
    pub crate_date: DateTime<Local>,
    pub finish_date: Option<DateTime<Local>>,
    pub modified_date: Option<DateTime<Local>>,
}
