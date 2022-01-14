mod model;

#[cfg(test)]
mod tests {
    use crate::model::entity::{Game, Kind};
    use std::fs::File;
    use std::io::{BufReader, Write};

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

    #[test]
    fn write_to_json_file_test() {
        let mut f = File::create("products.json").expect("Dosya oluşturmada hata");
        let mut games = vec![];
        games.push(Game {
            id: 2001,
            title: String::from("Age of Empires II"),
            kind: Kind::Rts,
        });
        games.push(Game {
            id: 1996,
            title: String::from("Starcraft II"),
            kind: Kind::Rts,
        });
        let data = serde_json::to_string(&games).expect("json dönüştürmede hata");
        let result = f
            .write_all(data.as_bytes())
            .expect("dosyaya yazma sırasında hata");
        assert_eq!(result, ());
    }

    #[test]
    fn read_json_from_file() {
        let f = File::open("products.json").expect("dosya açılırken hata");
        let reader = BufReader::new(f);
        let games: Vec<Game> = serde_json::from_reader(reader).expect("json okumada hata");
        assert_eq!(games.len(), 2);
        assert_eq!(games[0].id,2001);
    }
}
