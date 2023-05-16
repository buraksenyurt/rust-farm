use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub struct Database;

impl Database {
    pub async fn connect() -> surrealdb::Result<Surreal<Client>> {
        let db = Surreal::new::<Ws>("127.0.0.1:8002").await?;
        db.signin(Root {
            username: "scoth",
            password: "tiger",
        })
        .await?;
        db.use_ns("test").use_db("library").await?;
        Ok(db)
    }
}
