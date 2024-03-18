use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::Mutex;

fn main() {
    let now = std::time::Instant::now();

    let file_path = "large_file.txt";
    let output = Processor::find_words_count(file_path);
    let words_count = output.lock().unwrap();
    for (word, count) in words_count.iter() {
        println!("{}: {}", word, count);
    }

    println!("Geçen süre: {:?}", now.elapsed());
}

struct Processor;

impl Processor {
    pub fn find_words_count(file_path: &str) -> Mutex<HashMap<String, i32>> {
        let file = File::open(file_path).expect("file not found");
        let reader = io::BufReader::new(file);
        let output = Mutex::new(HashMap::new());

        reader
            .lines()
            .filter_map(Result::ok)
            .par_bridge()
            .for_each(|line| {
                let mut counts = output.lock().unwrap();
                line.split_whitespace()
                    .map(|word| {
                        word.trim_matches(|c: char| !c.is_alphanumeric())
                            .to_lowercase()
                    })
                    .for_each(|word| {
                        *counts.entry(word).or_insert(0) += 1;
                    });
            });

        output
    }
}
