use crate::state_manager::read_file;
use crate::work_item::factory::Factory;
use crate::work_item::status::Status;
use crate::{serializer, Size, Storage};
use actix_web::{web, Responder};
use serde_json::{Map, Value};
use std::str::FromStr;

/*
   Fonksiyon fiziki dosyada yer alan work item'ların JSON formatında çıktı olarak verilmesini sağlar.
   Dönüş tipi olarak kullanılan Responder bir trait'tir. Yani bu davranışı uygulayan tipler
   fonksiyondan dönülebilir. web üstünden örneklenen Json veri yapısı, bu trait'i uygulamaktadır.
   Buna göre get fonksiyonundan HTTP cevabı dönülebilmektedir.
*/

pub async fn get() -> impl Responder {
    // Öncelikle fiziki dosyadan Mission listesi çekiliyor.
    let missions: Map<String, Value> =
        read_file(Storage::get().as_str()).expect("JSON dosyası okunamadı");
    // Sonuç olarak dönülecek listeyi tutacak bir vector nesnesi oluşturuluyor
    let mut buffer = vec![];
    // Mission listesinin tüm key,value içerikleri dolaşılıyor.
    for (key, value) in missions {
        // Value ile gelen veri yapısındaki state ve value değerlerine bakılarak status ve size bilgileri alınıyor
        let status = Status::from_str(value.get("state").unwrap().as_str().unwrap()).unwrap();
        let size = Size::from(value.get("value").unwrap().as_u64().unwrap());
        // bu bilgilerden hareketler Mission nesnesi örnekleniyor ve
        let mission = Factory::create_work_item(status, &key, size).unwrap();
        // sonuç listesine ekleniyor
        buffer.push(mission);
    }
    // Dosyadan yüklenen mission içeriklerinden yararlanılarak WorkItemList nesnesi oluşturuluyor
    let work_items = serializer::work_item_list::WorkItemList::new(buffer);
    // Oluşturulan içerik JSON serileştirilerek olarak geriye dönülüyor.
    web::Json(work_items)
}
