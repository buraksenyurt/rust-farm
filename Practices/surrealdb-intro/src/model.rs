use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct Author<'a> {
    pub first_name: &'a str,
    pub mid_name: &'a str,
    pub last_name: &'a str,
    pub age: u8,
}

#[derive(Debug, Serialize)]
pub struct Book<'a> {
    pub title: &'a str,
    pub author: Author<'a>,
    pub page_size: u16,
    pub category: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub id: Thing,
}

#[derive(Debug, Deserialize)]
pub struct AuthorRecord {
    pub first_name: String,
    pub mid_name: String,
    pub last_name: String,
    pub age: u8,
}

#[derive(Debug, Deserialize)]
pub struct BookRecord {
    pub title: String,
    pub author: AuthorRecord,
    pub page_size: u16,
    pub category: String,
}
