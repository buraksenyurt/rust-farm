use serde::Serialize;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, Write};
use std::path::Path;

fn main() {
    convert();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn convert() {
    let mut books: Vec<Book> = Vec::new();
    if let Ok(lines) = read_lines("library_samples.csv") {
        let mut counter = 0;
        for line in lines {
            if counter == 0 {
                counter += 1;
                continue;
            }

            if let Ok(t) = line {
                let parts = t.split('|').collect::<Vec<&str>>();

                let book = Book {
                    title: parts[0].to_string(),
                    authors: parts[1].to_string(),
                    publisher: parts[2].to_string(),
                    location: parts[3].to_string(),
                };
                books.push(book);
            }
        }
    }

    let file_name = "library.json";
    let mut file = OpenOptions::new()
        //.append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .expect("Unable to open file");

    let books_json = serde_json::to_string_pretty(&books).unwrap();
    writeln!(file, "{}", books_json).expect("Serileştirme işleminde sorun oluştu");
}

#[derive(Serialize)]
struct Book {
    pub title: String,
    pub authors: String,
    pub publisher: String,
    pub location: String,
}
