pub mod auth;
pub mod developers;
pub mod games;

pub use auth::sign_in;
pub use developers::*;
pub use games::*;

use rocket::http::Status;
use sea_orm::DbErr;

#[derive(Responder)]
pub struct SuccessResponse<T>(pub (Status, T));

#[derive(Responder)]
pub struct ErrorResponse(pub (Status, String));

pub type Response<T> = Result<SuccessResponse<T>, ErrorResponse>;

impl From<DbErr> for ErrorResponse {
    fn from(err: DbErr) -> Self {
        ErrorResponse((Status::InternalServerError, err.to_string()))
    }
}
