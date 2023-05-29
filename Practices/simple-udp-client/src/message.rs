#[derive(serde::Serialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
}

impl Message {
    pub fn new(id: i32, content: String) -> Self {
        Self { id, content }
    }
}
