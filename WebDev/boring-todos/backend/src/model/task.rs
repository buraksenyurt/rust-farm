use crate::model::custom_error::Error;
use crate::model::database::Db;
use crate::model::task_state::TaskState;
use crate::security::user_context::UserContext;
use sqlb::{HasFields, Raw};

// Veri tabanındaki task tablosunu kod tarafındaki iz düşümü olan veri yapısı
#[derive(sqlx::FromRow, Debug)]
pub struct Task {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub state: TaskState,
}

// insert işleminde kullanıcı tarafından girilen verileri alan yapı olarak düşünülebilir.
// Örneğin id, create_date gibi alanlar otomatik postgresql tarafında oluşur. state için de varsayılan bir değer kullanılır
// sqlb::Fields kullanılması insert fonksiyonundaki payload.fields() çağrımını mümkün kılar.
// Bu sayede payload olarak ifade edilen TaskDao nesnesinin veri alanlarına göre otomatik oluşturulan
// sql insert sorgusunun alanları beslenebilir.
#[derive(sqlb::Fields, Debug, Clone)]
pub struct TaskDao {
    //pub user_id: Option<i64>,
    pub title: Option<String>,
    pub state: Option<TaskState>,
}

// Mac = Model Access Controller
// CRUD operasyonlarını ele alan kısım olarak düşünülebilir.
pub struct TaskMac;

impl TaskMac {
    const TABLE: &'static str = "task";
    const FIELDS: &'static [&'static str] = &["id", "user_id", "title", "state"];
}

// Model Access Controller fonksiyonları
impl TaskMac {
    pub async fn get_all(db: &Db, _uctx: &UserContext) -> Result<Vec<Task>, Error> {
        // let sql = "SELECT id,user_id,title,state FROM task ORDER By id DESC";
        // let query = sqlx::query_as(&sql);
        // let task_list = query.fetch_all(db).await?;

        // insert sorgusunun çalıştırılmasında olduğu gibi Select tipli sorgular içinde
        // Sql Builder tekniği kullanılabilir.
        let sql_builder = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::FIELDS)
            .order_by("!id");

        let task_list = sql_builder.fetch_all(db).await?;

        Ok(task_list)
    }

    pub async fn get_single(db: &Db, _uctx: &UserContext, record_id: i64) -> Result<Task, Error> {
        let sql_builder = sqlb::select()
            .table(Self::TABLE)
            .columns(Self::FIELDS)
            .and_where_eq("id", record_id);

        let task = sql_builder.fetch_one(db).await;

        handle(task, Self::TABLE, record_id)
    }

    pub async fn update(
        db: &Db,
        uctx: &UserContext,
        id: i64,
        data: TaskDao,
    ) -> Result<Task, Error> {
        // Veritabanındaki Task tablosuna güncelleme yapan kullanıcı ve zaman bilgisini eklemiştik.
        // Bu alanları Task veri yapısına koymadan da kullanmamız mümkün.
        // Bunun için Fields vektörüne ilgili alanları eklemek yeterli.
        let mut fields = data.fields();
        fields.push(("modify_user_id", uctx.user_id).into());
        fields.push(("modify_date", Raw("now()")).into());

        let sql_builder = sqlb::update()
            .table(Self::TABLE)
            .data(fields)
            .and_where_eq("id", id)
            .returning(Self::FIELDS);

        let updated_task = sql_builder.fetch_one(db).await;

        handle(updated_task, Self::TABLE, id)
    }

    pub async fn create(db: &Db, uctx: &UserContext, payload: TaskDao) -> Result<Task, Error> {
        // let sql =
        //     "INSERT INTO task (user_id,title) VALUES ($1,$2) returning id,user_id,title,state";
        // let query = sqlx::query_as::<_, Task>(&sql)
        //     .bind(payload.user_id)
        //     .bind(payload.title);

        //let created_task = query.fetch_one(db).await?;

        // Üstteki teknik yerine SQL Builder ile insert script'inin
        // nesne veri yapıları üzerinden oluşturulduğu aşağıdaki teknik tercih edilmeli.
        // Böylece elle script yazmak yerine var olan veri yapılarından bir tane oluşturulmasını
        // garanti edebiliriz.

        // İstersek payload olarak ifade edilen veri yapısında olmayan bir alan tanımını
        // manuel olarak aşağıdaki gibi ekleyebiliriz.
        let mut fields = payload.fields();
        fields.push(("user_id", uctx.user_id).into());

        let sql_builder = sqlb::insert()
            .table(Self::TABLE)
            .data(fields)
            .returning(Self::FIELDS);
        let created_task = sql_builder.fetch_one(db).await?;

        Ok(created_task)
    }

    pub async fn delete(db: &Db, _uctx: &UserContext, record_id: i64) -> Result<Task, Error> {
        let sql_builder = sqlb::delete()
            .table(Self::TABLE)
            .returning(Self::FIELDS)
            .and_where_eq("id", record_id);

        let deleted = sql_builder.fetch_one(db).await;

        handle(deleted, Self::TABLE, record_id)
    }
}

fn handle(
    result: Result<Task, sqlx::Error>,
    t: &'static str,
    record_id: i64,
) -> Result<Task, Error> {
    result.map_err(|serr| match serr {
        sqlx::Error::RowNotFound => Error::EntityNotFound(t, record_id.to_string()),
        other => Error::SqlxError(other),
    })
}

#[cfg(test)]
mod tests {
    use crate::model::custom_error::Error;
    use crate::model::database::init;
    use crate::model::task::{TaskDao, TaskMac};
    use crate::model::task_state::TaskState;
    use crate::security::user_context::get_user_from_token;

    #[tokio::test]
    async fn should_create_task_with_title_works() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;

        let candidate_task = TaskDao {
            //user_id: Some(4835),
            title: Some(String::from("Rust programlama için bir saat çalış")),
            state: None,
        };

        let user_context = get_user_from_token("10101").await?;

        let created_task = TaskMac::create(&db, &user_context, candidate_task.clone()).await?;
        assert!(created_task.id >= 1);
        assert_eq!(10101, created_task.user_id);
        assert_eq!(candidate_task.title.unwrap(), created_task.title);
        assert_eq!(TaskState::Ready, created_task.state);
        Ok(())
    }

    #[tokio::test]
    async fn should_get_single_task_works() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let user_context = get_user_from_token("9999").await?;

        let candidate_task = TaskDao {
            title: Some(String::from(
                "Akşam Red Alert oyun partisi var. Cipsleri al :)",
            )),
            state: None,
        };
        let created_task = TaskMac::create(&db, &user_context, candidate_task.clone()).await?;

        let result = TaskMac::get_single(&db, &user_context, created_task.id).await?;
        assert_eq!(result.id, created_task.id);

        Ok(())
    }

    #[tokio::test]
    async fn should_get_single_task_fails_with_wrong_id() -> Result<(), Box<dyn std::error::Error>>
    {
        let db = init().await?;
        let user_context = get_user_from_token("9999").await?;
        let result = TaskMac::get_single(&db, &user_context, 0).await;
        match result {
            Ok(_) => assert!(false, "Başarılı değil"),
            Err(Error::EntityNotFound(t, id)) => {
                assert_eq!("task", t);
                assert_eq!(0.to_string(), id);
            }
            other => assert!(false, "Başımız dertte {:?}", other),
        }
        Ok(())
    }

    #[tokio::test]
    async fn should_get_all_returns_more_than_one_task() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let user_context = get_user_from_token("10101").await?;
        let result = TaskMac::get_all(&db, &user_context).await?;
        assert!(result.len() > 0, "Görevler listesi");

        Ok(())
    }

    #[tokio::test]
    async fn should_update_some_task_works() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let user_context = get_user_from_token("9999").await?;

        let candidate_task = TaskDao {
            title: Some(String::from(
                "Akşam Red Alert oyun partisi var. Cipsleri al :)",
            )),
            state: None,
        };
        let created_task = TaskMac::create(&db, &user_context, candidate_task.clone()).await?;
        let update_candidate = TaskDao {
            title: Some(created_task.title),
            state: Some(TaskState::Completed),
        };

        let user_context = get_user_from_token("5001").await?;
        let updated_task =
            TaskMac::update(&db, &user_context, created_task.id, update_candidate).await?;
        assert_eq!(updated_task.state, TaskState::Completed);

        Ok(())
    }

    #[tokio::test]
    async fn should_delete_task_works() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let user_context = get_user_from_token("10101").await?;

        let candidate_task = TaskDao {
            title: Some(String::from(
                "Bu görev 5 saniye içinde kendi kendini yok edecektir. Boş şans Ethan Hunt :D",
            )),
            state: None,
        };
        let created_task = TaskMac::create(&db, &user_context, candidate_task.clone()).await?;

        let deleted_task = TaskMac::delete(&db, &user_context, created_task.id).await?;

        assert_eq!(deleted_task.id, created_task.id);
        assert_eq!(deleted_task.title, created_task.title);

        Ok(())
    }
}
