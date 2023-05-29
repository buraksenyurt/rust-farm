#[derive(serde::Deserialize)]
pub struct Message {
    pub id: i32,
    pub content: String,
}
