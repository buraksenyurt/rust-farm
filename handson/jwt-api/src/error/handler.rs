use crate::error::custom_error::CustomError;
use crate::error::error_response::ErrorResponse;
use log::error;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reject::MethodNotAllowed;
use warp::reply::{json, with_status, WithStatus};
use warp::{Rejection, Reply};

pub type Result<T> = std::result::Result<T, Rejection>;

pub fn reply_with_status(status: StatusCode, message: &str) -> WithStatus<impl Reply> {
    let json = json(&ErrorResponse {
        status: status.to_string(),
        message: message.to_string(),
    });
    with_status(json, status)
}

pub async fn catch_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(reply_with_status(StatusCode::NOT_FOUND, "Error Not Found"));
    }

    if let Some(e) = err.find::<CustomError>() {
        return match e {
            CustomError::UserExists(username) => Ok(reply_with_status(
                StatusCode::BAD_REQUEST,
                &format!("{} zaten mevcut", username),
            )),
            CustomError::InvalidCredentials => {
                Ok(reply_with_status(StatusCode::FORBIDDEN, &e.to_string()))
            }
            CustomError::AutoHeaderRequired
            | CustomError::NotAuthorized
            | CustomError::InvalidToken => {
                Ok(reply_with_status(StatusCode::UNAUTHORIZED, &e.to_string()))
            }
            CustomError::TokenCreation => Ok(reply_with_status(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
            )),
        };
    } else if err.find::<MethodNotAllowed>().is_some() {
        return Ok(reply_with_status(
            StatusCode::METHOD_NOT_ALLOWED,
            "Method Not Allowed",
        ));
    }

    error!("unhandled error: {:?}", err);
    Ok(reply_with_status(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal Server Error",
    ))
}
