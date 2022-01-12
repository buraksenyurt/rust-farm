mod model;

#[cfg(test)]
mod tests {
    use crate::model::entity::{Game, Kind};

    #[test]
    fn create_new_game_test() {
        let sc = Game {
            id: 1,
            title: String::from("Starcraft II"),
            kind: Kind::Rts,
        };
        let sc_info = format!("{}", sc);
        assert_eq!(sc_info, "1-Starcraft II(Rts)");
    }

    #[test]
    fn serialize_test() {
        let age_of = Game {
            id: 2001,
            title: String::from("Age of Empires II"),
            kind: Kind::Rts,
        };
        let serialized = serde_json::to_string(&age_of).expect("Serileştirme sırasında hata.");
        let expected = r#"{"id":2001,"title":"Age of Empires II","kind":"Rts"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn deserialize_test() {
        let serialized = r#"
            {
                "id":2001
                ,"title":"Age of Empires II"
                ,"kind":"Rts"
            }"#;
        let expected: Game = serde_json::from_str(serialized).unwrap();
        assert_eq!(expected.title, "Age of Empires II");
    }
}
