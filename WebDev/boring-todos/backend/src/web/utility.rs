use crate::model::database::Db;
use crate::security::user_context::{get_user_from_token, UserContext};
use crate::web::error::WebError;
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

// Header'da bir X-Auth-Token bilgisi olması halinde bunun içerisinden User Context'i yakaladığımız
// filtre fonksiyonudur.
pub fn add_auth(db: Arc<Db>) -> impl Filter<Extract = (UserContext,), Error = Rejection> + Clone {
    warp::any()
        .and(add_db(db))
        .and(warp::header::optional("X-Auth-Token"))
        .and_then(|db: Arc<Db>, xauth: Option<String>| async move {
            match xauth {
                Some(token) => {
                    let user_context = get_user_from_token(&db, &token).await?;
                    Ok::<UserContext, Rejection>(user_context)
                }
                None => Err(WebError::MissingXAuth.into()),
            }
        })
}
