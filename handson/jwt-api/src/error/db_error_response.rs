use serde::Serialize;

#[derive(Serialize)]
struct DbErrorResponse {
    message: String,
}
