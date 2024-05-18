use serde::{Deserialize, Serialize};
use shared::*;

#[derive(Serialize, Deserialize)]
pub struct CreateWorkItemResponse {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}
