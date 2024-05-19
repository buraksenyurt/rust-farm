use serde::{Deserialize, Serialize};
use shared::*;
use validator::Validate;
#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct CreateWorkItemRequest {
    #[validate(length(min = 10, max = 100, message = "Invalid title."))]
    pub title: String,
    #[validate(range(min = 1, max = 30, message = "Invalid duration range."))]
    pub duration: Option<u32>,
    pub duration_type: Option<DurationType>,
    pub size: Option<Size>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateStatusRequest {
    pub id: u32,
    pub new_status: Status,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MoveToArchiveRequest {
    pub id: u32,
}
