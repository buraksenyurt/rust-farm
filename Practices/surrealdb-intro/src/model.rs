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
    id: Thing,
}
