use crate::schema::{categories, games};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct Category {
    pub id: i32,
    pub title: String,
}

impl Display for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} - {}", self.id, self.title)
    }
}

#[derive(Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub title: &'a str,
}

#[derive(Debug, Queryable, Serialize)]
#[diesel(belongs_to(Category))]
pub struct Game {
    pub id: i32,
    pub category_id: i32,
    pub title: String,
    pub stars: i32,
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {} ({}*) Categroy {}",
            self.id, self.title, self.stars, self.category_id
        )
    }
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct UpsertGame<'a> {
    pub category_id: i32,
    pub title: &'a str,
    pub stars: i32,
}

impl<'a> UpsertGame<'a> {
    pub fn new(cat_id: i32, title: &'a str, stars: i32) -> Self {
        Self {
            category_id: cat_id,
            title,
            stars,
        }
    }
}

#[derive(Deserialize)]
pub struct NewGamePayload {
    pub category_id: i32,
    pub title: String,
    pub stars: i32,
}
