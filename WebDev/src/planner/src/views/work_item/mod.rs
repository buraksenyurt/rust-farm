use crate::views::path::Path;
use actix_web::web;
use log::info;

mod create;
mod get;

// workitem/create path'ine gelenleri karşılayan factory fonksiyonu
pub fn work_item_factory(app: &mut web::ServiceConfig) {
    info!("Work Item Factory");
    let base_path = Path {
        prefix: String::from("workitem"),
    };
    // yönlendirme tanımımız.
    // buna göre path üstünde title ve size bilgileri ile birlikte gelen bir HTTP Post talebi
    // söz konusu olursa create modülündeki create fonksiyonuna yönlendiriyoruz.
    app.route(
        &base_path.define(String::from("/create/{title}/{size}")),
        web::post().to(create::create),
    );

    // add adresine gelen taleplerde Body üstündeki JSON içeriği kullanılıyor.
    // Bunun için create modülündeki add fonksiyonunu deneysel olarak kullanmaktayız.
    app.route(
        &base_path.define(String::from("/add")),
        web::post().to(create::add),
    );

    // workitem/get adresine gelen Http Get taleplerini, get modülündeki get fonksiyonuna yönlendiriyoruz.
    // Bu talebe karşılık work item listesinin JSON formatını istemciye dönmekteyiz
    app.route(
        &base_path.define(String::from("/get")),
        web::get().to(get::get),
    );
}
