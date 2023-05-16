use crate::model::{Book, Record};
use crate::repository::Database;
use std::any::TypeId;
use surrealdb::engine::any::Any;

pub struct Controller;

impl Controller {
    pub async fn insert_book<'a>(book: Book<'a>) -> surrealdb::Result<Record> {
        let db = Database::connect().await?;
        let created = db.create("book").content(book).await?;
        Ok(created)
    }
    pub async fn get_books() -> surrealdb::Result<Vec<Record>> {
        let db = Database::connect().await?;
        let books: Vec<Record> = db.select("book").await?;
        Ok(books)
    }
    pub async fn get_book_by_id(id: &str) -> surrealdb::Result<Record> {
        let db = Database::connect().await?;
        let book: Record = db.select(("book", id)).await?;
        Ok(book)
    }
}
