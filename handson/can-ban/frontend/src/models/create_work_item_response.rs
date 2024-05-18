use serde::{Deserialize, Serialize};
use crate::models::*;

#[derive(Serialize, Deserialize)]
pub struct CreateWorkItemResponse {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}
