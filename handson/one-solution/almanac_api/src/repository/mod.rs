use crate::entity::game::{ActiveModel as GameActiveModel, Model as GameModel};
use crate::entity::prelude::Game;
use migration::Migrator;
use sea_orm::{entity::*, ActiveModelTrait, DatabaseConnection, DbErr, Set};
use sea_orm_migration::MigratorTrait;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameRepositoryError {
    #[error("Veritabanı ile ilgili hata")]
    DatabaseError(#[from] DbErr),
}
#[derive(Clone)]
pub struct GameRepository {
    db_con: Arc<DatabaseConnection>,
}

impl GameRepository {
    pub async fn new(db_conn: DatabaseConnection) -> Result<Self, GameRepositoryError> {
        Migrator::up(&db_conn, None)
            .await
            .map_err(|e| GameRepositoryError::DatabaseError(e))?;
        Ok(Self {
            db_con: Arc::new(db_conn),
        })
    }

    pub async fn add_game(
        &self,
        title: String,
        point: i32,
    ) -> Result<GameModel, GameRepositoryError> {
        let new_game = GameActiveModel {
            title: Set(title),
            point: Set(point),
            ..Default::default()
        };
        new_game
            .insert(self.db_con.as_ref())
            .await
            .map_err(|e| GameRepositoryError::DatabaseError(e))
    }

    pub async fn update_point(
        &self,
        id: i32,
        new_point: i32,
    ) -> Result<GameModel, GameRepositoryError> {
        let game = Game::find_by_id(id)
            .one(self.db_con.as_ref())
            .await
            .map_err(|e| GameRepositoryError::DatabaseError(e));

        let mut game_item = game.unwrap().unwrap();
        game_item.point = new_point;

        Ok(game_item)
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::GameRepository;
    use almanac_data::get_conn;
    use testcontainers::{clients, images};

    #[tokio::test]
    async fn should_add_new_game_test() {
        let docker = clients::Cli::default();
        let database = images::postgres::Postgres::default();
        let node = docker.run(database);
        let con_str = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        let connection = get_conn(con_str).await.unwrap();
        let repository = GameRepository::new(connection.clone()).await.unwrap();
        let title = "Ultra Super Bomberman".to_string();
        let point = 78;
        let game = repository.add_game(title.clone(), point).await.unwrap();
        assert_eq!(game.title, title.clone());
        assert_eq!(game.point, point);
    }

    #[tokio::test]
    async fn should_update_game_point_test() {
        let docker = clients::Cli::default();
        let database = images::postgres::Postgres::default();
        let node = docker.run(database);
        let con_str = &format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            node.get_host_port_ipv4(5432)
        );
        let connection = get_conn(con_str).await.unwrap();
        let repository = GameRepository::new(connection.clone()).await.unwrap();
        let title = "Ultra Super Bomberman II".to_string();
        let point = 78;
        let game = repository.add_game(title.clone(), point).await.unwrap();
        let new_point = 81;
        let updated_game = repository.update_point(game.id, new_point).await.unwrap();

        assert_eq!(updated_game.title, title.clone());
        assert_eq!(updated_game.point, new_point);
    }
}
