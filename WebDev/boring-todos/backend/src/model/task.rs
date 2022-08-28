use crate::model::custom_error::Error;
use crate::model::database::Db;
use crate::model::task_state::TaskState;
use sqlb::{HasFields};

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
}

// Mac = Model Access Controller
// CRUD operasyonlarını ele alan kısım olarak düşünülebilir.
pub struct TaskMac;

// Model Access Controller fonksiyonları
impl TaskMac {
    pub async fn get_all(db: &Db) -> Result<Vec<Task>, Error> {
        let sql = "SELECT id,user_id,title,state FROM task ORDER By id DESC";
        let query = sqlx::query_as(&sql);
        let task_list = query.fetch_all(db).await?;

        Ok(task_list)
    }

    pub async fn create(db: &Db, payload: TaskDao) -> Result<Task, Error> {
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
        fields.push(("user_id", 10101).into());

        let sql_builder = sqlb::insert()
            .table("task")
            .data(fields)
            .returning(&["id", "user_id", "title", "state"]);
        let created_task = sql_builder.fetch_one(db).await?;

        Ok(created_task)
    }
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;
    use crate::model::task::{TaskDao, TaskMac};
    use crate::model::task_state::TaskState;

    #[tokio::test]
    async fn should_get_all_returns_more_than_one_task() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let result = TaskMac::get_all(&db).await?;
        assert!(result.len() > 0, "Görevler listesi");

        Ok(())
    }

    #[tokio::test]
    async fn should_create_task_with_title_works() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;

        let candidate_task = TaskDao {
            //user_id: Some(4835),
            title: Some(String::from("Rust programlama için bir saat çalış")),
        };

        let created_task = TaskMac::create(&db, candidate_task.clone()).await?;
        assert!(created_task.id >= 1);
        assert_eq!(10101, created_task.user_id);
        assert_eq!(candidate_task.title.unwrap(), created_task.title);
        assert_eq!(TaskState::Ready, created_task.state);
        Ok(())
    }
}
