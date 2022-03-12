use crate::state_manager::{read_file, write_to_file};
use clap::{arg, Command};
use log::{warn};
use serde_json::{json, Map, Value};

mod state_manager;
mod work_item;

fn main() {
    env_logger::init();

    let file_path = format!("{}/states.json", env!("CARGO_MANIFEST_DIR"));

    let matches = Command::new("Planlayıcı")
        .version("v0.1.0")
        .author("Burak Selim Senyurt")
        .about("Kişisel kanban programı.")
        .subcommand(
            Command::new("create")
                .about("Yeni görev oluşturur.")
                .short_flag('c')
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
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("create", argmatchs)) => {
            let title: String = argmatchs.value_of_t("TITLE").unwrap();
            let business_value: u16 = argmatchs.value_of_t("VALUE").unwrap();
            let mut state: Map<String, Value> =
                read_file(file_path.as_str()).expect("JSON dosyası okunamadı");
            let mission = json!({
                "value": business_value,
                "status": "Ready",
            });
            state.insert(title, mission);
            write_to_file(file_path.as_str(), &mut state).expect("Dosya yazma sırasında hata");
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
