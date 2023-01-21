use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Claim {
    pub subject: String,
    pub role: String,
    pub expiration: usize,
}
