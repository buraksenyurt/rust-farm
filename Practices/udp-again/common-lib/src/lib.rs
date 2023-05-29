#[derive(serde::Serialize, serde::Deserialize)]
pub struct PlayerState {
    pub pos: Position,
    pub player_id: i32,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
