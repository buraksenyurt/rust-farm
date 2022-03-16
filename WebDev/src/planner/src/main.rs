use crate::action::Action;
use crate::work_item::size::Size;
use clap::{arg, Command};
use log::warn;
use serde_json::{json, Map, Value};
use state_manager::{read_file, write_to_file};
use std::str::FromStr;
use storage::Storage;

pub mod action;
pub mod processor;
pub mod state_manager;
pub mod storage;
pub mod work_item;

fn main() {
    env_logger::init();
    let matches = Command::new("Planlayıcı")
        .version("v0.1.0")
        .author("Burak Selim Senyurt")
        .about("Kişisel kanban programı.")
        .subcommand(
            Command::new("action")
                .about("Görev aksiyonu")
                .short_flag('a')
                .arg(
                    arg!(<ACTION>)
                        .help("Komut")
                        .short('c')
                        .long("command")
                        .required(true),
                )
                .arg(
                    arg!(<TITLE>)
                        .help("Görev adı")
                        .short('t')
                        .long("title")
                        .required(true),
                )
                .arg(
                    arg!(<VALUE>)
                        .help("Görevin Değeri")
                        .short('v')
                        .long("value")
                        .possible_values(vec!["1", "3", "5", "8", "13"])
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("action", argmatchs)) => {
            let action = Action::from_str(argmatchs.value_of("ACTION").unwrap());
            let title: String = argmatchs.value_of_t("TITLE").unwrap();
            let business_value: u16 = argmatchs.value_of_t("VALUE").unwrap();
            let mut state: Map<String, Value> =
                read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
            let mission = json!({
                "value": business_value,
                "status": "Ready",
            });
            state.insert(title, mission);
            write_to_file(Storage::get().as_str(), &mut state).expect("Dosya yazma sırasında hata");
        }
        _ => warn!("Doğru komut bulunamadı."),
    }

    // let cleaning = Factory::create_work_item(Status::Ready, "Odayı temizle", 8);
    // match cleaning {
    //     Some(Mission::Ready(m)) => m.create(&m.header.title, m.header.value),
    //     Some(Mission::Doing(m)) => m.get(&m.header.title),
    //     Some(Mission::Completed(m)) => info!("{} tamalanmış", &m.header.title),
    //     _ => {}
    // }
}
