use crate::body::Body;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "src")]
    pub source: String,
    #[serde(rename = "dest")]
    pub destination: String,
    pub body: Body,
}
