use crate::state_manager::read_file;
use crate::Storage;
use actix_web::{web, Responder};

/*
   Fonksiyon fiziki dosyada yer alan work item'ların JSON formatında çıktı olarak verilmesini sağlar.
   Dönüş tipi olarak kullanılan Responder bir trait'tir. Yani bu davranışı uygulayan tipler
   fonksiyondan dönülebilir. web üstünden örneklenen Json veri yapısı, bu trait'i uygulamaktadır.
   Buna göre get fonksiyonundan HTTP cevabı dönülebilmektedir.
*/

pub async fn get() -> impl Responder {
    let workitems = read_file(Storage::get().as_str());
    web::Json(workitems)
}
