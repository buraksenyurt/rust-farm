use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameAddedEvent {
    id: i32,
    title: String,
    point: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GamePointChangedEvent {
    id: i32,
    title: String,
    old_point: i32,
    new_point: i32,
}
