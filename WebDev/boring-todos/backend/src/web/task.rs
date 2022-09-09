use crate::model::database::Db;
use crate::model::task::{TaskDao, TaskMac};
use crate::security::user_context::UserContext;
use crate::web::utility::{add_auth, add_db, to_json_response};
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

    // Tüm görevleri çeken route tanımı
    // HTTP GET ile çalışır
    // api/tasks
    let all_tasks = task_route
        .and(warp::get())
        .and(warp::path::end())
        .and(filters.clone())
        .and_then(get_all_tasks);

    // Tek bir task çekmek için gerekli route tanımı
    // HTTP GET ile çalışır
    // api/tasks/35 gibi parametrik
    let get_single_task = task_route
        .and(warp::get())
        .and(filters.clone())
        .and(warp::path::param())
        .and_then(get_task);

    // yeni task oluşturmak için gerekli route tanımı
    // HTTP POST ile çalışır ve gövdede JSON formatı kullanılır
    let create_task = task_route
        .and(warp::post())
        .and(filters.clone())
        .and(warp::body::json())
        .and_then(create_task);

    // Bir görevi silmek için kullanılacak route tanımı
    // HTTP DELETE ile çalışır
    // api/tasks/35 gibi parametrikdir
    let delete_task = task_route
        .and(warp::delete())
        .and(filters.clone())
        .and(warp::path::param())
        .and_then(delete_task);

    // Görev bilgisini güncellemek için gerekli route tanımı
    // HTTP PATCH ile çalışır
    // api/tasks/10 gibi parametrikdir
    let update_task = task_route
        .and(warp::patch())
        .and(filters.clone())
        .and(warp::path::param())
        .and(warp::body::json())
        .and_then(update_task);

    // Tüm route tanımlarının bağlandığı yer
    all_tasks
        .or(get_single_task)
        .or(create_task)
        .or(delete_task)
        .or(update_task)
}

// Tüm görev listesini JSON formatında(başarılı olursa) döndüren fonksiyon
async fn get_all_tasks(db: Arc<Db>, user_context: UserContext) -> Result<Json, warp::Rejection> {
    log::info!("Get All Task metodu çağrıldı");
    let tasks = TaskMac::get_all(&db, &user_context).await?;
    to_json_response(tasks)
}

// Belli bir ID değerindeki görevi çeken fonksiyon
async fn get_task(
    db: Arc<Db>,
    user_context: UserContext,
    record_id: i64,
) -> Result<Json, warp::Rejection> {
    let task = TaskMac::get_single(&db, &user_context, record_id).await?;
    to_json_response(task)
}

// Yeni bir görev ekleme için kullanılan fonksiyon
async fn create_task(
    db: Arc<Db>,
    user_context: UserContext,
    payload: TaskDao,
) -> Result<Json, warp::Rejection> {
    let created_task = TaskMac::create(&db, &user_context, payload).await?;
    to_json_response(created_task)
}

// Bir görevi silmek için kullanılan tokio fonksiyonu
async fn delete_task(
    db: Arc<Db>,
    user_context: UserContext,
    record_id: i64,
) -> Result<Json, warp::Rejection> {
    let deleted = TaskMac::delete(&db, &user_context, record_id).await?;
    to_json_response(deleted)
}

// Görev içeriğini güncellemek için kullanılan tokio fonksiyonu
async fn update_task(
    db: Arc<Db>,
    user_context: UserContext,
    record_id: i64,
    payload: TaskDao,
) -> Result<Json, warp::Rejection> {
    let updated = TaskMac::update(&db, &user_context, record_id, payload).await?;
    to_json_response(updated)
}

#[cfg(test)]
mod test {
    use crate::init;
    use crate::model::task::Task;
    use crate::web::handle_web_error;
    use crate::web::task::task_router;
    use anyhow::{Context, Result};
    use serde_json::{from_str, from_value, json, Value};
    use std::str::from_utf8;
    use std::sync::Arc;
    use warp::Filter;
    use crate::model::task_state::TaskState;

    #[tokio::test]
    async fn should_tasks_http_get_works() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone());

        let response = warp::test::request()
            .method("GET")
            .header("X-Auth-Token", "10101")
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

    #[tokio::test]
    async fn should_create_task_and_get_works() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone());

        let body = json!({"title":"10 Km yürü"});

        let response = warp::test::request()
            .method("POST")
            .header("X-Auth-Token", "10101")
            .path("/api/tasks")
            .json(&body)
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200, "Yeni görev ekleme");

        let body = from_utf8(response.body())?;
        let mut body: Value = from_str(body)
            .with_context(|| "Mesaj içeriği JSON formatında ters serileştirilemedi.")?;
        let data = body["data"].take();
        let data: Task = from_value(data)?;
        let path = format!("/api/tasks/{}", data.id);

        let response = warp::test::request()
            .method("GET")
            .header("X-Auth-Token", "10101")
            .path(&path)
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
        let get_data = body["data"].take();
        let get_data: Task = from_value(get_data)?;

        assert_eq!(data.id, get_data.id, "Tek görev ekleme ve çekme");

        Ok(())
    }

    #[tokio::test]
    async fn should_create_task_and_delete_works() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone());

        let body = json!({"title":"Bu görev az sonra kendisini yok edecektir"});

        let response = warp::test::request()
            .method("POST")
            .header("X-Auth-Token", "10101")
            .path("/api/tasks")
            .json(&body)
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200, "Yeni görev ekleme");

        let body = from_utf8(response.body())?;
        let mut body: Value = from_str(body)
            .with_context(|| "Mesaj içeriği JSON formatında ters serileştirilemedi.")?;
        let data = body["data"].take();
        let data: Task = from_value(data)?;
        let path = format!("/api/tasks/{}", data.id);

        let response = warp::test::request()
            .method("DELETE")
            .header("X-Auth-Token", "10101")
            .path(&path)
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
        let deleted_data = body["data"].take();
        let deleted_data: Task = from_value(deleted_data)?;

        assert_eq!(data.id, deleted_data.id, "Tek görev ekleme ve silme");

        Ok(())
    }

    #[tokio::test]
    async fn should_create_task_and_update_works() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone());

        let body = json!({"title":"Ubuntu versiyonunu yükselt."});

        let response = warp::test::request()
            .method("POST")
            .header("X-Auth-Token", "10101")
            .path("/api/tasks")
            .json(&body)
            .reply(&api)
            .await;

        assert_eq!(response.status(), 200, "Yeni görev ekleme");

        let body = from_utf8(response.body())?;
        let mut body: Value = from_str(body)
            .with_context(|| "Mesaj içeriği JSON formatında ters serileştirilemedi.")?;
        let data = body["data"].take();
        let data: Task = from_value(data)?;
        let update_data = json!({"title":data.title,"state":"Completed"});

        let path = format!("/api/tasks/{}", data.id);

        let response = warp::test::request()
            .method("PATCH")
            .header("X-Auth-Token", "10101")
            .path(&path)
            .json(&update_data)
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
        let updated_data = body["data"].take();
        let updated_data: Task = from_value(updated_data)?;

        assert_eq!(updated_data.state, TaskState::Completed, "Tek görev ekleme ve silme");

        Ok(())
    }

    #[tokio::test]
    async fn should_tasks_http_get_throw_rejection() -> Result<()> {
        let db = init().await?;
        let db = Arc::new(db);
        let api = task_router("api", db.clone()).recover(handle_web_error);

        let response = warp::test::request()
            .method("GET")
            .path("/api/tasks")
            .reply(&api)
            .await;

        assert_eq!(response.status(), 400, "Recover kontrol testi");

        Ok(())
    }
}
