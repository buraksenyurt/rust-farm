use serde::Serialize;

#[derive(Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub list_price: f32,
}
