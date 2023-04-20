use crate::argument::{Command, List};
use crate::builder::{check_and_create_folder, download_many, show_help};
use std::env;
use std::process::exit;
use std::str::FromStr;

mod argument;
mod builder;
mod photo;
mod test;

#[tokio::main]
async fn main() {
    if !check_and_create_folder() {
        println!("Photos klasörü oluşturulamadı");
        exit(1);
    }
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 | 4 => match Command::from_str(args[1].as_str()) {
            Ok(Command::Single(_)) => {}
            Ok(Command::Many(_)) => {
                let page_number = u8::from_str(args[2].as_str());
                let count = u8::from_str(args[3].as_str());
                if page_number.is_ok() && count.is_ok() {
                    download_many(List::new(page_number.unwrap(), count.unwrap())).await;
                } else {
                    println!("many argümanlarında hata var");
                    show_help();
                }
            }
            _ => {}
        },
        _ => {
            show_help();
        }
    }
}
