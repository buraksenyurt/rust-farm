use crate::game::Game;
use std::collections::HashMap;
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

pub fn cluster_by_release_year(games: Vec<Game>) -> HashMap<u16, Vec<Game>> {
    let mut clusters = HashMap::new();
    for game in games {
        clusters
            .entry(game.release_year)
            .or_insert_with(Vec::new)
            .push(game);
    }
    clusters
}
pub fn cluster_by_producer(games: &[Game]) -> HashMap<&str, Vec<usize>> {
    let mut clusters = HashMap::new();
    for (index, game) in games.iter().enumerate() {
        clusters
            .entry(game.producer.as_str())
            .or_insert_with(Vec::new)
            .push(index);
    }
    clusters
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

    #[test]
    fn get_cluster_by_release_year_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let clustered = cluster_by_release_year(games.unwrap());
        assert!(clustered.get(&1996).is_some());
    }

    #[test]
    fn get_cluster_by_producer_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let binding = games.unwrap();
        let clustered = cluster_by_producer(binding.as_slice().clone());
        assert!(clustered.get("Nintendo").is_some());
    }
}
