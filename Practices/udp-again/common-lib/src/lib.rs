use std::fmt::{Display, Formatter};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct GameState {
    pub players: Vec<PlayerState>,
}

impl Default for GameState {
    fn default() -> Self {
        Self { players: vec![] }
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct PlayerState {
    pub pos: Position,
    pub player_id: i32,
}

impl PlayerState {
    pub fn new(x: f32, y: f32, z: f32, player_id: i32) -> Self {
        Self {
            player_id,
            pos: Position::new(x, y, z),
        }
    }
}

impl Display for PlayerState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {}, Position {}", self.player_id, self.pos)
    }
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}:{}:{})", self.x, self.y, self.z)
    }
}

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[cfg(test)]
mod test {
    use crate::PlayerState;
    use bincode::{deserialize, serialize};

    #[test]
    pub fn should_bincode_encoding_works_test() {
        let john_doe = PlayerState::new(10.5, 11.6, -28.0, 1202356);
        let encoded = serialize(&john_doe).unwrap();
        let decoded: PlayerState = deserialize(&encoded).unwrap();
        assert_eq!(decoded, john_doe);
    }
}
