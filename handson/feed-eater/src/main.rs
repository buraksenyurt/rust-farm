use crate::command::{get_command, Command};
use crate::reader::load_feeds_from_file;
use dotenv::dotenv;
use std::env;
use std::env::args;

mod command;
mod feed;
mod reader;

fn main() {
    dotenv().expect("Environment dosyası okunamadı");
    let source_path = env::var("DATASOURCE").expect("Env dosyasında datasource içeriği yok");
    let feeds = load_feeds_from_file(source_path);
    let args: Vec<String> = args().collect();
    let command = get_command(args);
    match command {
        Command::Top(value) => {
            reader::get(&feeds, value);
        }
        Command::SelectAll => {
            reader::get(&feeds, None);
        }
        Command::GetFeeds => {
            feeds.iter().for_each(|f| println!("{}", f));
        }
        Command::Undefined => {
            println!("Kullanabileceğiniz komutlar\n");
            println!("top value - belirtilen sayıda içeriği çeker.");
            println!("all - feed içeriklerinin tamamını çeker.");
            println!("feeds - güncel feed listesini verir");
        }
    }
}
