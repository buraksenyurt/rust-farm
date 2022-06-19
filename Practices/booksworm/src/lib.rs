use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

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
pub fn add_book(title: String, authors: String, is_read: bool) -> bool {
    let book = Book::new(title, authors, is_read);
    println!("{}", book.to_string());
    true
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
