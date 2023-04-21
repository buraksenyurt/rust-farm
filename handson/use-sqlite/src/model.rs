use crate::schema::*;
use diesel::{Insertable, Queryable};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Category {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = categories)]
pub struct CategoryNew<'a> {
    pub title: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryJson {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Game {
    pub id: i32,
    pub title: String,
    pub stars: i32,
    pub category_id: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = games)]
pub struct GameNew<'a> {
    pub title: &'a str,
    pub stars: i32,
    pub category_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameJson {
    pub title: String,
    pub stars: i32,
    pub category_id: i32,
}
