use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub unit_price: f32,
    pub quantity: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Invoice {
    pub title: String,
    pub serial_number: String,
    pub customer: String,
    pub total_amount: f32,
    pub line_items: Vec<Item>,
}
