use crate::model::database::Db;
use crate::security::error::TokenError;

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

// Sembolik olarak yazdığımız ve bir token içinden kullanıcı id bilgisini çeken fonksiyon
pub async fn get_user_from_token(_db: &Db, token: &str) -> Result<UserContext, TokenError> {
    match token.parse::<i64>() {
        Ok(id) => Ok(UserContext::new(id)),
        Err(_) => Err(TokenError::InvalidToken(token.to_string())),
    }
}
