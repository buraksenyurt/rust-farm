use serde::{Deserialize, Serialize};
use shared::*;

#[derive(Serialize, Deserialize)]
pub struct CreateWorkItemRequest {
    pub title: String,
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
}
