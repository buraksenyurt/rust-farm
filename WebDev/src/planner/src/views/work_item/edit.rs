use crate::action::Action;
use crate::processor::run;
use crate::serializer::work_item::WorkItem;
use crate::state_manager::read_file;
use crate::work_item::factory::Factory;
use crate::work_item::status::Status;
use crate::Storage;
use actix_web::{web, HttpResponse};
use log::info;
use serde_json::{Map, Value};
use std::str::FromStr;

// Work Item'ı düzenlemek için kullanılan fonksiyondur
pub async fn edit(work_item: web::Json<WorkItem>) -> HttpResponse {
    info!("edit fonksiyonuna JSON içeriği geldi\n{:#?}", work_item);
    let mut state: Map<String, Value> =
        read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
    // JSON içeriğindeki title ve size bilgilerinin bir klonu alınır.
    let title = &work_item.title;
    let size = work_item.size;
    // Title bilgisinin veri deposunda(ki örneğimizde fiziki dosya) olup olmadığına bakılır.
    match state.get(title) {
        // Eğer title içeriğine sahip bir görev varsa
        Some(wi) => {
            // Öncelike status bilgisi çekilir
            let status = Status::from_str(wi.get("state").unwrap().as_str().unwrap()).unwrap();
            // Bu statü bilgisine göre, gelen title ve size bilgileri de kullanılarak bir Mission oluşturulur
            match Factory::create_work_item(status, title, size) {
                Some(m) => {
                    // mission, run fonksiyonuna Edit modda gönderilir nitekim bir düzenleme söz konusu
                    run(m, Action::Edit, &mut state);
                    HttpResponse::Ok().json(format!(
                        "{} başlıklı görevin büyüklüğü {} olarak değiştirildi. Durum bir sonraki seviyeye çekildi.",
                        title, size
                    ))
                }
                None => HttpResponse::BadRequest().json("Bir sorun var.".to_string()),
            }
        }
        // Yoksa eğer HTTP 404 NotFound dönülür ve içinde bir bilgi verilir.
        None => HttpResponse::NotFound().json(format!("{} başlıklı bir görev bulunamadı", title)),
    }
}
