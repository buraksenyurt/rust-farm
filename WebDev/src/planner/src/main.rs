//use crate::action::Action;
//use crate::processor::run;
//use crate::work_item::factory::Factory;
use crate::work_item::size::Size;
//use crate::work_item::status::Status;
use actix_web::dev::Service;
use actix_web::{App, HttpServer};
//use clap::{arg, Command};
use log::{info, warn};
//use serde_json::{Map, Value};
use state_manager::write_to_file;
//use std::str::FromStr;
use storage::Storage;

pub mod action;
pub mod processor;
mod serializer;
pub mod state_manager;
pub mod storage;
mod views;
pub mod work_item;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Main fonksiyonu localhost:8000 portundan hizmet veren bir web server haline getiriliyor.
    env_logger::init();
    HttpServer::new(|| {
        // uygulama nesnesi oluşturulurken view'ları ele alacak factory fonksiyonu da atanıyor.
        // wrap_fn ile middleware'e closure kullanarak bir fonksiyon ekledik.
        // request ve app_router parametreleri ile gelen ve giden mesajları yakalamak pekala mümkün.
        // ilk olarak workitem path'ine doğru gelen talepleri yakalıyoruz.
        // Gelen talebin içerisindeki token bilgisini de kontrol amaçlı yardımcı fonksiyona yolluyoruz.
        let app = App::new()
            .wrap_fn(|request, app_router| {
                if request.path().contains("/workitem/") {
                    match views::token::process_token(&request) {
                        Ok(_) => info!("Token bilgisi geçerli"),
                        Err(message) => info!("token hatası: {}", message),
                    }
                }
                // Şimdilik token bilgisinin geçerli olup olmadığına bakmaksızın akışı devam ettirdik.
                let future = app_router.call(request);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory);
        warn!("Web sunucusu oluşturuldu");
        app
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await

    // let matches = Command::new("Planlayıcı")
    //     .version("v0.1.0")
    //     .author("Burak Selim Senyurt")
    //     .about("Kişisel kanban programı.")
    //     .subcommand(
    //         Command::new("set")
    //             .about("Görev kontrol")
    //             .short_flag('s')
    //             .arg(
    //                 arg!(<ACTION>)
    //                     .help("Aksiyon")
    //                     .short('a')
    //                     .long("ACTION")
    //                     .possible_values(["create", "edit", "delete", "get"])
    //                     .required(true),
    //             )
    //             .arg(
    //                 arg!(<TITLE>)
    //                     .help("Görev adı")
    //                     .short('t')
    //                     .long("title")
    //                     .required(true),
    //             )
    //             .arg(
    //                 arg!(<SIZE>)
    //                     .help("Görevin büyüklüğü.(T-Shirt Size)")
    //                     .short('v')
    //                     .long("value")
    //                     .possible_values(["1", "3", "5", "8", "13"]),
    //             ),
    //     )
    //     .get_matches();
    //
    // match matches.subcommand() {
    //     Some(("set", argmatchs)) => {
    //         let action = Action::from_str(argmatchs.value_of("ACTION").unwrap()).unwrap();
    //         let title = argmatchs.value_of("TITLE").unwrap();
    //         let size: u64 = argmatchs.value_of_t("SIZE").unwrap();
    //         let size = Size::from(size);
    //         let mut state: Map<String, Value> =
    //             read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
    //
    //         let (status, size) = match state.get(title) {
    //             Some(wi) => (
    //                 Status::from_str(wi.get("state").unwrap().as_str().unwrap()).unwrap(),
    //                 Size::from(wi.get("value").unwrap().as_u64().unwrap()),
    //             ),
    //             None => (Status::Ready, size),
    //         };
    //
    //         let work_item = Factory::create_work_item(status, title, size).unwrap();
    //         run(work_item, action, &mut state);
    //     }
    //     _ => warn!("Parametrelerde hata var."),
    // }
}
