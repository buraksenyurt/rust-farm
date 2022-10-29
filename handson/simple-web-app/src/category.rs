use crate::product::Product;
use serde::Serialize;

#[derive(Serialize)]
pub struct Category {
    pub id: i32,
    pub title: String,
    pub products: Vec<Product>,
}
