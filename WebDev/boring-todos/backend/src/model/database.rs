use sqlx::postgres::PgPoolOptions;
use sqlx::{query, Executor, Pool, Postgres};
use std::fs;

pub type Db = Pool<Postgres>;
const HOST: &str = "localhost";
const ROOT_DB: &str = "postgres";
const USER: &str = "scoth";
const PASSWORD: &str = "tiger";
const TASK_DB: &str = "task_force_db";
const TASK_DB_PATH: &str = "sql/";
const TASK_DB_INIT: &str = "sql/00-init-db.sql";

pub async fn init() -> Result<Db, sqlx::Error> {
    let con_str = format!("postrgres://{}:{}@{}/{}", USER, PASSWORD, HOST, ROOT_DB);
    let root_db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&con_str)
        .await?;

    let content = fs::read_to_string(TASK_DB_INIT).map_err(|err| {
        println!("Dosya okumada hata. Sebep {:?}", err);
        err
    })?;

    let commands: Vec<&str> = content.split(";").collect();

    for command in commands {
        match root_db.execute(command).await {
            Ok(_) => (),
            Err(e) => println!("SQL komutu çalıştırılırken hata. Sebep {:?}", e),
        }
    }

    let con_str = format!("postrgres://{}:{}@{}/{}", USER, PASSWORD, HOST, TASK_DB);
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&con_str)
        .await
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;

    #[tokio::test]
    async fn model_database_init() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        Ok(())
    }
}
