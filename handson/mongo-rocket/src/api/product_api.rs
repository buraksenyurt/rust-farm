use crate::model::product_model::Product;
use crate::repository::db::Db;
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/product", data = "<p>")]
pub fn create_product(db: &State<Db>, p: Json<Product>) -> Result<Json<InsertOneResult>, Status> {
    let data = Product {
        id: None,
        title: p.title.to_owned(),
        price: p.price,
        stock_level: p.stock_level,
        category: p.category.to_owned(),
    };
    let created = db.create_product(data);
    match created {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/product/<object_id>")]
pub fn get_product(db: &State<Db>, object_id: String) -> Result<Json<Product>, Status> {
    if object_id.is_empty() {
        return Err(Status::BadRequest);
    };
    let product = db.get_product(&object_id);
    match product {
        Ok(p) => Ok(Json(p)),
        Err(_) => Err(Status::NotFound),
    }
}
