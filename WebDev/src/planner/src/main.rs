use crate::action::Action;
use crate::processor::run;
use crate::work_item::factory::Factory;
use crate::work_item::size::Size;
use crate::work_item::status::Status;
use clap::{arg, Command};
use log::warn;
use serde_json::{Map, Value};
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
            Command::new("set")
                .about("Görev kontrol")
                .short_flag('s')
                .arg(
                    arg!(<ACTION>)
                        .help("Aksiyon")
                        .short('a')
                        .long("ACTION")
                        .possible_values(["create", "edit", "delete", "get"])
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
                    arg!(<SIZE>)
                        .help("Görevin büyüklüğü.(T-Shirt Size)")
                        .short('v')
                        .long("value")
                        .possible_values(["1", "3", "5", "8", "13"]),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", argmatchs)) => {
            let action = Action::from_str(argmatchs.value_of("ACTION").unwrap()).unwrap();
            let title = argmatchs.value_of("TITLE").unwrap();
            let size: u64 = argmatchs.value_of_t("SIZE").unwrap();
            let size = Size::from(size);
            let mut state: Map<String, Value> =
                read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");

            let (status, size) = match state.get(title) {
                Some(wi) => (
                    Status::from_str(wi.get("state").unwrap().as_str().unwrap()).unwrap(),
                    Size::from(wi.get("value").unwrap().as_u64().unwrap()),
                ),
                None => (Status::Ready, size),
            };

            let work_item = Factory::create_work_item(status, title, size).unwrap();
            run(work_item, action, &mut state);
        }
        _ => warn!("Parametrelerde hata var."),
    }
}
