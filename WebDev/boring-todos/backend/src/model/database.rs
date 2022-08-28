use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, Pool, Postgres};
use std::fs;
use std::path::PathBuf;

pub type Db = Pool<Postgres>;
const HOST: &str = "localhost";
//const ROOT_DB: &str = "postgres";
const USER: &str = "scoth";
const PASSWORD: &str = "tiger";
const TASK_DB: &str = "task_force_db";
const SCRIPTS_ROOT_PATH: &str = "sql/";
const TASK_DB_INIT_FILE: &str = "sql/00-init-db.sql";

pub async fn init() -> Result<Db, sqlx::Error> {
    //let con_str = format!("postrgres://{}:{}@{}/{}", USER, PASSWORD, HOST, ROOT_DB);
    let con_str = format!("postrgres://{}:{}@{}/{}", USER, PASSWORD, HOST, TASK_DB);
    let root_db = PgPoolOptions::new()
        .max_connections(1)
        .connect(&con_str)
        .await?;

    let mut paths: Vec<PathBuf> = fs::read_dir(SCRIPTS_ROOT_PATH)?
        .into_iter()
        .filter_map(|d| d.ok().map(|d| d.path()))
        .collect();
    paths.sort();

    for p in paths {
        if let Some(p) = p.to_str() {
            if p.ends_with(".sql") && p != TASK_DB_INIT_FILE {
                let content = fs::read_to_string(p).map_err(|err| {
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
            }
        }
    }

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&con_str)
        .await
}

#[cfg(test)]
mod tests {
    use crate::model::database::init;
    use sqlx::Executor;

    #[tokio::test]
    async fn should_database_initialized_succesfully() -> Result<(), Box<dyn std::error::Error>> {
        let db = init().await?;
        let result = db.execute("SELECT COUNT(id) FROM task").await?;
        assert!(result.rows_affected() > 0, "Eklenen toplam satır sayısı");

        Ok(())
    }
}
