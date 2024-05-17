use crate::types::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateWorkItemRequest {
    pub title: String,
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
