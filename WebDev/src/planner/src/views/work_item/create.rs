use crate::action::Action;
use crate::processor::run;
use crate::state_manager::read_file;
use crate::work_item::factory::Factory;
use crate::work_item::status::Status;
use crate::{Size, Storage};
use actix_web::HttpRequest;
use serde_json::{Map, Value};
use std::str::FromStr;

// Bu view yeni bir work item oluşturmak için HttpRequest üstünden gelen bilgileri kullanır.
pub async fn create(request: HttpRequest) -> String {
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
    // work item oluşturulur.
    let work_item = Factory::create_work_item(status, title, size).expect("Work Item Create error");
    run(work_item, Action::Create, &mut state);
    format!("{} başlıklı görev oluşturuldu", title)
}
