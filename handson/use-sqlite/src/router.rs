use crate::database::open_connection;
use crate::model::Category;
use crate::schema::categories::dsl::categories;
use actix_web::{Error, HttpResponse};
use diesel::RunQueryDsl;

pub async fn get_categories() -> Result<HttpResponse, Error> {
    let conn = &mut open_connection();
    let category_list = categories
        .load::<Category>(conn)
        .expect("Kategoriler yüklenemedi");
    //let response =serde_json::to_string(&category_list.to_vec())?;
    Ok(HttpResponse::Ok().json(&category_list.to_vec()))
}
