use serde::{Deserialize, Serialize};
use crate::models::Status;

#[derive(Serialize, Deserialize)]
pub struct UpdateStatusRequest {
    pub id: u32,
    pub new_status: Status,
}
