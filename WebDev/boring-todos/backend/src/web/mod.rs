mod error;
mod task;
mod utility;

use crate::model::database::Db;
use crate::web::error::{WebError, WebErrorMessage};
use serde_json::json;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::web::task::task_router;

// Web sunucusunu başlatma işlemini üstlenen fonksiyondur
pub async fn run_web_server(web_folder: &str, port: u16, db: Arc<Db>) -> Result<(), WebError> {
    // İlk önce web klasörünün var olup olmadığına bakılır. Yoksa Error basılır

    if !Path::new(web_folder).exists() {
        return Err(WebError::WebFolderNotFound(web_folder.to_string()));
    }
    log::info!("Kullanılacak path {}", web_folder);
    //region Api ve Statik içerik kullanımı

    // API yönlendirmesi için route tanımlamalarını alıyoruz
    let apis=task_router("api",db);

    // İlk etapta statik bir içerik basılacağı için aşağıdaki hazırlıklar yapılır.
    // Parametre olarak gelen web klasörü içerisindeki index.html kullanılır.
    let content = warp::fs::dir(web_folder.to_string());
    let rootx = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    // Api taleplerini ve root klasöre talep gelirse de statik içeriğin basılmasını sağlıyoruz.
    let web_site = apis.or(content).or(rootx);
    // Yönlendirmeler ile ilgili bir recover mekanizması da eklendi.
    let routes = web_site.recover(handle_web_error);
    println!("127.0.0.1:{} adresinden sunucu hizmeti açılacak", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    //endregion Api ve Statik içerik kullanımı

    Ok(())
}

async fn handle_web_error(err: Rejection) -> Result<impl Reply, Infallible> {
    log::error!("{:?}", err);

    let user_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.kind.to_string(),
        None => "Bilinmeyen Hata(Unkown)".to_string(),
    };

    let response = json!({ "ErrorMessage": user_message });
    let response = warp::reply::json(&response);

    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::BAD_REQUEST,
    ))
}
