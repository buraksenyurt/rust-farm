use crate::schema::{categories, games};
use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Category {
    pub id: i32,
    pub title: String,
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub title: &'a str,
}

#[derive(Debug, Queryable)]
#[diesel(belongs_to(Category))]
pub struct Game {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub stars: i32,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct UpsertGame {
    pub title: String,
    pub stars: i32,
    pub category_id: i32,
}
