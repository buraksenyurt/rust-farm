use crate::dto::Game;
use crate::repository::{GameRepository, GameRepositoryError};
use thiserror::Error;

#[derive(Clone)]
pub struct Service {
    repository: GameRepository,
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Repository hatası")]
    RepositoryError(#[from] GameRepositoryError),
}

impl Service {
    pub fn new(repository: GameRepository) -> Self {
        Self { repository }
    }

    pub async fn add_game(&self, title: String, point: i32) -> Result<Game, ServiceError> {
        let added_game = self
            .repository
            .add_game(title, point)
            .await
            .map_err(|e| ServiceError::RepositoryError(e))
            .unwrap();

        let game = Game {
            id: added_game.id,
            title: added_game.title,
            point: added_game.point,
        };
        Ok(game)
    }

    pub async fn update_point(&self, game_id: i32, new_point: i32) -> Result<Game, ServiceError> {
        let updated_game = self
            .repository
            .update_point(game_id, new_point)
            .await
            .map_err(|e| ServiceError::RepositoryError(e))
            .unwrap();

        let game = Game {
            id: updated_game.id,
            title: updated_game.title,
            point: updated_game.point,
        };

        Ok(game)
    }
}
