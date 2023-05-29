#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
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

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
