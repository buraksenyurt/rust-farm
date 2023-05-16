use crate::model::{Book, BookRecord, Record};
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
    pub async fn get_records() -> surrealdb::Result<Vec<Record>> {
        let db = Database::connect().await?;
        let records: Vec<Record> = db.select("book").await?;
        Ok(records)
    }
    pub async fn get_record_by_id(id: &str) -> surrealdb::Result<Record> {
        let id1=id.split(':').collect::<Vec<&str>>()[1];
        let db = Database::connect().await?;
        let record: Record = db.select(("book",id1) ).await?;
        Ok(record)
    }
    pub async fn get_books() -> surrealdb::Result<Vec<BookRecord>> {
        let db = Database::connect().await?;
        let books: Vec<BookRecord> = db.select("book").await?;
        Ok(books)
    }
    pub async fn get_book_by_id(id: &str) -> surrealdb::Result<BookRecord> {
        let id1=id.split(':').collect::<Vec<&str>>()[1];
        let db = Database::connect().await?;
        let book: BookRecord = db.select(("book", id1)).await?;
        Ok(book)
    }
}
