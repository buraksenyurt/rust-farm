mod task;
mod utility;

use crate::model::database::Db;
use std::path::Path;
use std::sync::Arc;
use warp::Filter;

// Web sunucusunu başlatma işlemini üstlenen fonksiyondur
pub async fn run_web_server(web_folder: &str, port: u16, _db: Arc<Db>) -> Result<(), Error> {
    // İlk önce web klasörünün var olup olmadığına bakılır. Yoksa Error basılır

    if !Path::new(web_folder).exists() {
        return Err(Error::WebFolderNotFound(web_folder.to_string()));
    }
    println!("Kullanılacak path {}", web_folder);
    //region Statik içerik kullanımı

    // İlk etapta statik bir içerik basılacağı için aşağıdaki hazırlıklar yapılır.
    // Parametre olarak gelen web klasörü içerisindeki index.html kullanılır.
    let content = warp::fs::dir(web_folder.to_string());
    let rootx = warp::get()
        .and(warp::path::end())
        .and(warp::fs::file(format!("{}/index.html", web_folder)));
    let web_site = content.or(rootx);
    let routes = web_site;
    println!("127.0.0.1:{} adresinden sunucu hizmeti açılacak", port);
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    //endregion Statik içerik kullanımı

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web sunucu başlatma hatası. {0} klasörü bulunamadı")]
    WebFolderNotFound(String),
}
