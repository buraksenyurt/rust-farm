use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}
