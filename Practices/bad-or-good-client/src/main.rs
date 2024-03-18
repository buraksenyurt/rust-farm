pub struct StringOps;

impl StringOps {
    pub fn calc_worst(words: Vec<String>) -> String {
        let mut output = String::new();
        for word in words {
            output += &word;
        }
        output
    }

    pub fn calc_green(words: Vec<String>) -> String {
        // Önce toplam uzunluğu bulalım.
        let len = words.iter().map(|w| w.len()).sum();
        // Bulunun uzunluk kadar boyuta sahip bir String tanımlayalım
        let mut output = String::with_capacity(len);
        for word in words {
            output.push_str(&word);
        }
        output
    }
}

fn main() {
    let words = vec![
        "C#".to_string(),
        "Python".to_string(),
        "Go".to_string(),
        "Rust".to_string(),
        "C++".to_string(),
        "C".to_string(),
        "Assembly".to_string(),
        "Cobol".to_string(),
        "Delphi".to_string(),
    ];
    let words_clone = words.clone();
    let output = StringOps::calc_worst(words);
    let output = StringOps::calc_green(words_clone);
}
