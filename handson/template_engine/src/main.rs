use std::collections::HashMap;
use std::io::{stdin, BufRead};
use template_engine::prelude::*;

fn main() {
    println!("Bir HTML şablonu ekleyin.");
    println!("<b>Hoşgeldin [[user_name]]. Şu anki seviyen [[level]]</b>\ngibi...");

    let mut context = HashMap::new();
    context.insert("user_name".to_string(), "Persival".to_string());
    context.insert("level".to_string(), "Pro".to_string());

    for line in stdin().lock().lines() {
        match get_content_type(&line.unwrap().clone()) {
            ContentType::TemplateVariable(content) => {
                let html = generate_html_template_variable(content, context.clone());
                println!("{}", html);
            }
            ContentType::Literal(text) => println!("{}", text),
            ContentType::Tag(TagType::Loop) => println!("Bir döngü söz konusu."),
            ContentType::Tag(TagType::If) => println!("If bloğu söz konusu."),
            ContentType::Unknown => println!("Anlaşılmadı."),
        }
    }
}
