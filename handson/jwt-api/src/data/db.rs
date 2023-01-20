use crate::model::user::User;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

// Örnekte veritabanı taklidi yapan tipimiz bir HashMap.
// Kullanıcı bilgisini taşıyan User veri modelini kullanmakta.
// key olarak username değerleri kullanılacak.
// thread-safe erişim sağlayacak şekilde tanımlanmıştır.
pub type UsersDb = Arc<Mutex<HashMap<String, User>>>;

pub fn add_users_db(
    users_db: UsersDb,
) -> impl Filter<Extract = (UsersDb,), Error = Infallible> + Clone {
    warp::any().map(move || users_db.clone())
}
