use serde::{Deserialize, Serialize};

// Claim veri yapısındaki sub ve exp gibi isimlendirmeleri
// subject, expiration gibi kullanınca Bearer token çözümlenmesinde problem yaşadım!!!
#[derive(Deserialize, Serialize, Debug)]
pub struct Claim {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}
