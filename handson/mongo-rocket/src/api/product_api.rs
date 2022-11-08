use crate::model::product_model::Product;
use crate::repository::db::Db;
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
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

#[get("/products")]
pub fn get_products(db: &State<Db>) -> Result<Json<Vec<Product>>, Status> {
    let products = db.get_products();
    match products {
        Ok(p) => Ok(Json(p)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/product/<object_id>")]
pub fn delete_product(db: &State<Db>, object_id: String) -> Result<Json<&str>, Status> {
    if object_id.is_empty() {
        return Err(Status::BadRequest);
    }
    let result = db.delete_product(&object_id);
    match result {
        Ok(response) => {
            if response.deleted_count == 1 {
                return Ok(Json("Product deleted"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/product/<object_id>", data = "<payload>")]
pub fn update_product(
    db: &State<Db>,
    object_id: String,
    payload: Json<Product>,
) -> Result<Json<Product>, Status> {
    if object_id.is_empty() {
        return Err(Status::BadRequest);
    }
    let data = Product {
        id: Some(ObjectId::parse_str(&object_id).unwrap()),
        title: payload.title.to_owned(),
        price: payload.price,
        stock_level: payload.stock_level,
        category: payload.category.to_owned(),
    };
    let update_result = db.update_product(&object_id, data);
    match update_result {
        Ok(u) => {
            if u.matched_count == 1 {
                let updated = db.get_product(&object_id);
                return match updated {
                    Ok(p) => Ok(Json(p)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
