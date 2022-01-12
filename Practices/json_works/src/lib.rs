mod model;

#[cfg(test)]
mod tests {
    use crate::model::entity::{Game, Kind};

    #[test]
    fn create_new_game_test() {
        let sc = Game {
            id: 1,
            title: String::from("Starcraft II"),
            kind: Kind::RTS,
        };
        let sc_info = format!("{}", sc);
        assert_eq!(sc_info, "1-Starcraft II(Rts)");
    }
}
