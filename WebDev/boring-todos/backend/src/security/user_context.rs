use thiserror::Error as ThisError;

// Task kaydeden kullanıcı bilgisini güvenli bir token içinden almayı planlıyoruz.
// Bu token içeriğindeki user_id bilgisi task oluşturan kullanıcı bilgisi olarak kullanılabilir.
// En basit haliyle aşağıdaki veri yapısı ile temsil edebiliriz
pub struct UserContext {
    pub user_id: i64,
}

impl UserContext {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

#[derive(ThisError, Debug)]
pub enum TokenError {
    #[error("Geçersiz token {0}")]
    InvalidToken(String),
}

// Sembolik olarak yazdığımız ve bir token içinden kullanıcı id bilgisini çeken fonksiyon
pub async fn get_user_from_token(token: &str) -> Result<UserContext, TokenError> {
    match token.parse::<i64>() {
        Ok(id) => Ok(UserContext::new(id)),
        Err(_) => Err(TokenError::InvalidToken(token.to_string())),
    }
}
