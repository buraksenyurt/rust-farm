use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize)]
pub struct Author<'a> {
    first_name: &'a str,
    mid_name: &'a str,
    last_name: &'a str,
    age: u8,
}

#[derive(Debug, Serialize)]
pub struct Book<'a> {
    title: &'a str,
    author: Author<'a>,
    page_size: u16,
    category: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    id: Thing,
}
