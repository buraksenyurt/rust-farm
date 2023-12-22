use crate::game::Game;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::PathBuf;

mod game;

pub fn read_file(file_path: &str) -> io::Result<Vec<Game>> {
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

fn organize_by<Key, F>(games: Vec<Game>, mut key_function: F) -> HashMap<Key, Vec<Game>>
where
    Key: Eq + Hash + Clone,
    F: FnMut(&Game) -> Key,
{
    let mut result_set = HashMap::new();
    for game in games {
        let key = key_function(&game);
        result_set.entry(key).or_insert_with(Vec::new).push(game);
    }
    result_set
}

pub fn organize_by_release_year(games: Vec<Game>) -> HashMap<u16, Vec<Game>> {
    organize_by(games, |g| g.release_year)
}
pub fn organize_by_producer(games: Vec<Game>) -> HashMap<String, Vec<Game>> {
    organize_by(games, |g| g.producer.clone())
}

pub fn save<K>(data_set: HashMap<K, Vec<Game>>, dir: PathBuf) -> Result<(), Error>
where
    K: Display,
{
    for (key, games) in data_set {
        let file_name = key.to_string().replace(|c: char| !c.is_alphanumeric(), "_");
        let file_path = dir.join(format!("{file_name}.txt"));
        let mut file = File::create(file_path)?;

        for game in games {
            writeln!(file, "{}", game)?;
        }
    }
    Ok(())
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
        let clustered = organize_by_release_year(games.unwrap());
        assert!(clustered.get(&1996).is_some());
    }

    #[test]
    fn get_cluster_by_producer_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let clustered = organize_by_producer(games.unwrap());
        assert!(clustered.get("Nintendo").is_some());
    }

    #[test]
    fn save_producer_based_clusters_works_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let data_set = organize_by_producer(games.unwrap());
        let result = save(data_set, PathBuf::new());
        assert!(result.is_ok());
    }

    #[test]
    fn save_release_year_based_clusters_works_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let data_set = organize_by_release_year(games.unwrap());
        let result = save(data_set, PathBuf::new());
        assert!(result.is_ok());
    }
}
