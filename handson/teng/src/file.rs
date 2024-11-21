use crate::model::Invoice;
use std::fs;

pub fn load_json(file_name: &str) -> Invoice {
    let content = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    serde_json::from_str(&content).expect("JSON was not well-formatted")
}

pub fn load_template(path: &str) -> String {
    fs::read_to_string(path).expect("Template file was not found")
}
