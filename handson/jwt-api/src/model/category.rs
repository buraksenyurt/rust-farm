use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Category {
    pub id: i32,
    pub title: String,
}
