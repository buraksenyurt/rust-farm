use crate::action::Action;
use crate::processor::run;
use crate::serializer::work_item::WorkItem;
use crate::state_manager::read_file;
use crate::work_item::factory::Factory;
use crate::work_item::status::Status;
use crate::{Size, Storage};
use actix_web::{web, HttpRequest, HttpResponse};
use log::{info, warn};
use serde_json::{Map, Value};
use std::str::FromStr;

// Bu sefer HTTP Post talebine ait mesajın JSON içeriğini kullanarak bir Work Item eklenmekte.
pub async fn add(work_item: web::Json<WorkItem>) -> HttpResponse {
    info!("Add fonksiyonuna JSON içeriği geldi\n{:#?}", work_item);
    let mut state: Map<String, Value> =
        read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
    let title = work_item.title.clone();
    let size = work_item.size;
    match state.get(&title) {
        Some(_) => {
            warn!("Gelen görev başlığı zaten mevcut");
            HttpResponse::Found().json(Value::String("Bu görev zaten tanımlı".to_string()))
        }
        None => {
            let mission = Factory::create_work_item(Status::Ready, &title, size)
                .expect("Work Item Create error");
            run(mission, Action::Create, &mut state);
            HttpResponse::Ok().json(Value::String("Work Item Oluşturuldu".to_string()))
        }
    }
}

// Bu fonksiyon yeni bir work item oluşturmak için HttpRequest üstünden gelen bilgileri kullanır.
pub async fn create(request: HttpRequest) -> String {
    info!("Create modülüne talep geldi. {:#?}", request);
    // State bilgileri JSON dosyasından yüklenir
    let mut state: Map<String, Value> =
        read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
    // Http talebi üstünden title bilgisi okunur
    let title = request.match_info().get("title").unwrap();
    // Http talebi üstünden size bilgisi çekilir.
    let size: u64 = request
        .match_info()
        .get("size")
        .unwrap()
        .to_string()
        .parse::<u64>()
        .unwrap();
    let size = Size::from(size);
    // statü bilgisi için önce JSON dosyasından yüklenen veri kümesine bakılır.
    // Eğer varsa durumuna bakılarak bir değişiklik yapılır.
    let (status, size) = match state.get(title) {
        Some(wi) => (
            Status::from_str(wi.get("state").unwrap().as_str().unwrap()).unwrap(),
            Size::from(wi.get("value").unwrap().as_u64().unwrap()),
        ),
        None => (Status::Ready, size),
    };
    let size_ref = size;

    // work item oluşturulur.
    let mission = Factory::create_work_item(status, title, size).expect("Work Item Create error");
    run(mission, Action::Create, &mut state);
    format!(
        "{} başlıklı iş, {} büyüklüğünde oluşturuldu.",
        title, size_ref
    )
}
