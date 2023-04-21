use crate::database::open_connection;
use crate::model::Category;
use crate::schema::categories::dsl::categories;
use diesel::prelude::*;

mod database;
mod model;
mod schema;

fn main() {
    let conn = &mut open_connection();
    let category_list = categories
        .load::<Category>(conn)
        .expect("Kategoriler yüklenemedi");

    for c in category_list {
        println!("{}", c.title);
    }
}
