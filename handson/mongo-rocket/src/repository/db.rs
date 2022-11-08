use crate::model::product_model::Product;
use dotenv::dotenv;
use mongodb::bson::{doc, extjson::de::Error, oid::ObjectId};
use mongodb::results::{DeleteResult, InsertOneResult};
use mongodb::sync::{Client, Collection};
use std::env;

pub struct Db {
    col: Collection<Product>,
}

impl Db {
    pub fn init() -> Self {
        dotenv().ok();
        let server_address = match env::var("MONGOHOST") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Environment variable error"),
        };
        let client = Client::with_uri_str(server_address).expect("Client connection error");
        let database = client.database("AdventureWorks");
        let col: Collection<Product> = database.collection("Product");
        Db { col }
    }

    pub fn create_product(&self, p: Product) -> Result<InsertOneResult, Error> {
        let product_doc = Product {
            id: None,
            title: p.title,
            price: p.price,
            stock_level: p.stock_level,
            category: p.category,
        };
        let product = self
            .col
            .insert_one(product_doc, None)
            .ok()
            .expect("Error creating product");

        Ok(product)
    }

    pub fn get_product(&self, id: &String) -> Result<Product, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let product = self
            .col
            .find_one(filter, None)
            .ok()
            .expect("Error getting product");
        Ok(product.unwrap())
    }

    pub fn delete_product(&self, id: &String) -> Result<DeleteResult, Error> {
        let object_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id":object_id};
        let product = self
            .col
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting product");
        Ok(product)
    }
}
