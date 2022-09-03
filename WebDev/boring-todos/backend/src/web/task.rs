use crate::model::database::Db;
use crate::model::task::TaskMac;
use crate::security::user_context::UserContext;
use crate::web::utility::{add_auth, add_db};
use serde_json::json;
use std::sync::Arc;
use warp::reply::Json;
use warp::Filter;

// Görevler için REST metotlarını karşılayan router tanımlamalarının ayarlandığı fonksiyon
pub fn task_router(
    base_path: &'static str,
    db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // api/tasks gibi bir adresi ifade ediyoruz
    let task_route = warp::path(base_path).and(warp::path("tasks"));
    // db ve sembolik authentication işlerine ait filtreleri uçuca ekliyoruz.
    // aşağıdaki REST operasyonlarında ortaklaşa kullanılacaklar.
    let filters = add_db(db.clone()).and(add_auth(db.clone()));

    let all_tasks = task_route
        .and(warp::get())
        .and(warp::path::end())
        .and(filters.clone())
        .and_then(get_all_tasks);

    all_tasks
}

// Tüm görev listesini JSON formatında(başarılı olursa) döndüren fonksiyon
async fn get_all_tasks(db: Arc<Db>, user_context: UserContext) -> Result<Json, warp::Rejection> {
    let tasks = TaskMac::get_all(&db, &user_context).await.unwrap();
    let response = json!({ "data": tasks });
    Ok(warp::reply::json(&response))
}

#[cfg(test)]
mod test {
    use crate::init;
    use crate::model::task::Task;
    use crate::web::task::task_router;
    use anyhow::{Context, Result};
    use serde_json::{from_str, from_value, Value};
    use std::str::from_utf8;
    use std::sync::Arc;

    #[tokio::test]
    async fn should_tasks_http_get_works() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone());

        let response = warp::test::request()
            .method("GET")
            .path("/api/tasks")
            .reply(&api)
            .await;

        assert_eq!(
            response.status(),
            200,
            "Görev listesi için HTTP Get çağrımı"
        );

        let body = from_utf8(response.body())?;
        let mut body: Value = from_str(body)
            .with_context(|| "Mesaj içeriği JSON formatında ters serileştirilemedi.")?;
        let data = body["data"].take();
        let data: Vec<Task> = from_value(data)?;

        assert!(data.len() > 0, "Görev sayısı");

        Ok(())
    }
}
