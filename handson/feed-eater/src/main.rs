use crate::command::{get_command, Command};
use crate::reader::{add_feed, load_feeds_from_file, write_all_feeds_again};
use dotenv::dotenv;
use std::env;
use std::env::args;
use std::io::stdin;

mod command;
mod feed;
mod reader;

fn main() {
    dotenv().expect("Environment dosyası okunamadı");
    let source_path = env::var("DATASOURCE").expect("Env dosyasında datasource içeriği yok");
    let feeds = load_feeds_from_file(source_path.clone());
    let args: Vec<String> = args().collect();
    let command = get_command(args);
    match command {
        Command::Add(feed) => {
            let _ = add_feed(source_path, feed);
        }
        Command::Delete => {
            println!("Silmek istediğiniz feed'in numarasını verin");
            feeds
                .iter()
                .enumerate()
                .for_each(|(idx, f)| println!("{} - {}", idx, f));

            let mut line = String::new();
            if stdin().read_line(&mut line).is_ok() {
                if let Ok(v) = line.trim().parse::<usize>() {
                    let new_feeds = feeds
                        .into_iter()
                        .enumerate()
                        .filter(|(idx, _)| *idx != v)
                        .map(|(_, feed)| feed)
                        .collect();
                    write_all_feeds_again(source_path, new_feeds);
                    println!("{}. feed silindi!", v);
                }
            }
        }
        Command::GetFeeds => {
            feeds.iter().for_each(|f| println!("{}", f));
        }
        Command::SelectAll => {
            reader::get(&feeds, None);
        }
        Command::Top(value) => {
            reader::get(&feeds, value);
        }
        Command::Undefined => {
            println!("Kullanabileceğiniz komutlar\n");
            println!("top value - belirtilen sayıda içeriği çeker.");
            println!("all - feed içeriklerinin tamamını çeker.");
            println!("feeds - güncel feed listesini verir.");
            println!("add feed_adı feed_adresi - yeni bir feed ekler.");
            println!("del value - Feed'i silmek için kullanılır.");
        }
    }
}
