use crate::messages::{
    CreateGameRequest, CreateGameResponse, UpdateGamePointRequest, UpdateGamePointResponse,
};
use crate::service::{Service, ServiceError};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::post, routing::put, Extension, Json, Router};
use serde_json::json;
use std::net::SocketAddr;
use std::str::FromStr;

pub struct Server{
    pub address: String
}

impl Server {
    pub async fn run(&self, service: Service) {
        let add_game_router = Router::new().route("/", post(create_game));
        let update_game_router = Router::new().route("/", put(update_game_point));
        let api_router = Router::new()
            .nest("/games", add_game_router)
            .nest("/games", update_game_router);
        let app = Router::new()
            .nest("/api", api_router)
            .layer(Extension(service));
        let address = SocketAddr::from_str(self.address.as_str());
        axum::Server::bind(&address.unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap()
    }
}

async fn create_game(
    Extension(service): Extension<Service>,
    Json(request): Json<CreateGameRequest>,
) -> impl IntoResponse {
    let result = service.add_game(request.title, request.point).await;
    match result {
        Ok(created) => (
            StatusCode::CREATED,
            Json(json!(CreateGameResponse {
                id: created.id,
                title: created.title,
                point: created.point
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        ),
    }
}

async fn update_game_point(
    Extension(service): Extension<Service>,
    Json(request): Json<UpdateGamePointRequest>,
) -> impl IntoResponse {
    let result = service.update_point(request.id, request.new_point).await;
    match result {
        Ok(updated) => (
            StatusCode::CREATED,
            Json(json!(UpdateGamePointResponse {
                id: updated.id,
                title: updated.title,
                new_point: updated.point,
                old_point: 0 // ???
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error":e.to_string()})),
        ),
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ServiceError::RepositoryError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        let body = Json(json!({ "error": error_message }));
        (status, body).into_response()
    }
}
