use crate::model::ModelError;
use crate::security::TokenError;

#[derive(thiserror::Error, Debug)]
pub enum WebError {
    #[error("Web sunucu başlatma hatası. {0} klasörü bulunamadı.")]
    WebFolderNotFound(String),
    #[error("Doğrulama geçersiz. X-Auth-Token header bilgisi bulunamadı.")]
    MissingXAuth,
}
#[derive(Debug)]
pub struct WebErrorMessage {
    pub message: String,
    pub kind: &'static str,
}

impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
    pub fn throw(k: &'static str, msg: String) -> warp::Rejection {
        warp::reject::custom(WebErrorMessage {
            kind: k,
            message: msg,
        })
    }
}

impl From<WebError> for warp::Rejection {
    fn from(other: WebError) -> Self {
        WebErrorMessage::throw("web::Error", format!("{:?}", other))
    }
}

impl From<ModelError> for warp::Rejection {
    fn from(other: ModelError) -> Self {
        WebErrorMessage::throw("model::Error", format!("{:?}", other))
    }
}

impl From<TokenError> for warp::Rejection {
    fn from(other: TokenError) -> Self {
        WebErrorMessage::throw("security::Error", format!("{:?}", other))
    }
}
