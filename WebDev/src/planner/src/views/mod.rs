use actix_web::web;
use log::info;

mod auth;
mod path;
mod work_item;

pub fn views_factory(app: &mut web::ServiceConfig) {
    info!("Factory nesneleri bağlanıyor");
    // authentication ve work_item view'ları için kullanılan factory nesneleri
    // Actix App nesnesi ile ilişkilendirilir.
    auth::auth_factory(app);
    work_item::work_item_factory(app);
}
