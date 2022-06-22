use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub authors: String,
    pub is_read: bool,
}

impl Book {
    pub fn new(title: String, authors: String, is_read: bool) -> Self {
        Self {
            title,
            authors,
            is_read,
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.title, self.authors)
    }
}

#[wasm_bindgen]
pub fn add_book(title: String, authors: String, is_read: bool) -> String {
    let book = Book::new(title, authors, is_read);
    let data = serde_json::to_string(&book).unwrap();
    data
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
