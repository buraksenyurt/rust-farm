use crate::body::Body;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    #[serde(rename = "src")]
    source: String,
    #[serde(rename = "dest")]
    destination: String,
    body: Body,
}
