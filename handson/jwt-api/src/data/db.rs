use crate::model::user::User;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

// Örnekte veritabanı taklidi yapan tipimiz bir HashMap.
// Kullanıcı bilgisini taşıyan User veri modelini kullanmakta.
// key olarak username değerleri kullanılacak.
// thread-safe erişim sağlayacak şekilde tanımlanmıştır.
pub type UsersDb = Arc<Mutex<HashMap<String, User>>>;
