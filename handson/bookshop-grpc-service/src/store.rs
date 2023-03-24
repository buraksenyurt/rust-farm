use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Clone)]
pub struct Book {
    pub book_id: i32,
    pub title: String,
    pub authors: String,
    pub category: String,
    pub unit_price: f64,
}

pub fn load_books() -> Option<Vec<Book>> {
    if let Ok(f) = File::open("./books.json") {
        let reader = BufReader::new(f);
        if let Ok(books) = serde_json::from_reader(reader) {
            return Some(books);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use crate::store::load_books;

    #[test]
    pub fn should_load_books_works() {
        let actual = load_books();
        assert!(actual.is_some());
        if let Some(books) = actual {
            assert!(books.len() > 0);
        }
    }
}
