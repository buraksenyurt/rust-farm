use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MoveToArchiveRequest {
    pub id: u32,
}
