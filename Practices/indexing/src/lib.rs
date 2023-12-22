use crate::game::Game;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

mod game;

fn read_file(file_path: &str) -> io::Result<Vec<Game>> {
    let mut games = Vec::new();
    let f = File::open(file_path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        let columns: Vec<&str> = line.split('|').collect();
        if let [id, title, average_point, release_year, producer, platform] = &columns[..] {
            let g = Game {
                id: id.parse().unwrap(),
                title: title.to_string(),
                average_point: average_point.parse().unwrap(),
                release_year: release_year.parse().unwrap(),
                producer: producer.to_string(),
                platform: platform.to_string(),
            };
            games.push(g);
        }
    }

    Ok(games)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_games_file_works_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        assert!(games.is_ok());
    }
}
