use crate::views::path::Path;
use actix_web::web;

mod create;

// workitem/create path'ine gelenleri karşılayan factory fonksiyonu
pub fn work_item_factory(app: &mut web::ServiceConfig) {
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
}
