#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::Controller;
    use crate::model::{Author, Book};
    use crate::repository::Database;
    use std::any::{Any, TypeId};

    #[tokio::test]
    async fn should_connecting_db_works_test() {
        let db = Database::connect().await;
        assert!(db.is_ok())
    }

    #[tokio::test]
    async fn should_insert_book_works_test() {
        let book = Book {
            title: "Practical System Programming for Rust Developers",
            author: Author {
                first_name: "Prabhu",
                mid_name: "",
                last_name: "Eshwarla",
                age: 46,
            },
            page_size: 365,
            category: "Programming",
        };
        let created = Controller::insert_book(book).await;
        assert!(created.is_ok());
    }

    #[tokio::test]
    async fn should_get_all_records_works_test() {
        let records = Controller::get_records().await;
        assert!(records.is_ok());
        match records {
            Ok(list) => assert!(list.len() > 0),
            _ => {}
        }
    }

    #[tokio::test]
    async fn should_get_record_by_id_works_test() {
        // let book = Book {
        //     title: "Practical System Programming for Rust Developers",
        //     author: Author {
        //         first_name: "Prabhu",
        //         mid_name: "",
        //         last_name: "Eshwarla",
        //         age: 46,
        //     },
        //     page_size: 365,
        //     category: "Programming",
        // };
        // let created = Controller::insert_book(book).await;
        let record = Controller::get_record_by_id("k31lx6i4u3ld1ujiokd9").await;
        assert!(record.is_ok());
    }

    #[tokio::test]
    async fn should_get_all_books_works_test() {
        let books = Controller::get_books().await;
        assert!(books.is_ok());
        match books {
            Ok(list) => assert!(list.len() > 0),
            _ => {}
        }
    }

    #[tokio::test]
    async fn should_get_book_by_id_works_test() {
        let book = Controller::get_book_by_id("k31lx6i4u3ld1ujiokd9").await;
        assert!(book.is_ok());
        assert_eq!(
            book.unwrap().title,
            "Practical System Programming for Rust Developers"
        );
    }
}
