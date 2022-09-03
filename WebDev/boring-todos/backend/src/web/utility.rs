use crate::model::database::Db;
use crate::security::user_context::{get_user_from_token, UserContext};
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection};

// Her rest operasyonun (görev listesini çekmek, yeni görev eklemek, silmek ve güncellemek)
// veritabanı nesnesine ve şimdilik sembolik olarak eklenen authentication mekanizmasına
// gereksinim var. Bunları warp motoruna güvenli bir şekilde klonlayarak eklemeyi sağlayan
// yardımcı fonksiyonlar aşağıdaki gibi yazılabilir. Her bir fonksiyon warp motoru için bir filtre
// nesnesi döndürmektedir. Bu fonksiyonlar ortama yeniden kullanılabilirlik kazandırır.
pub fn add_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn add_auth(_db: Arc<Db>) -> impl Filter<Extract = (UserContext,), Error = Rejection> + Clone {
    warp::any().and_then(|| async {
        Ok::<UserContext, Rejection>(get_user_from_token("10101").await.unwrap())
    })
}
