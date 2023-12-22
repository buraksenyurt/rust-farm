use crate::game::Game;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Error, Write};

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
pub fn cluster_by_producer(games: Vec<Game>) -> HashMap<String, Vec<Game>> {
    let mut clusters = HashMap::new();
    for game in games {
        let producer = game.producer.clone();
        clusters
            .entry(game.producer)
            .or_insert_with(Vec::new)
            .push(Game {
                id: game.id,
                title: game.title,
                average_point: game.average_point,
                release_year: game.release_year,
                producer,
                platform: game.platform,
            });
    }
    clusters
}

pub fn save_release_year_based_clusters(clusters: HashMap<u16, Vec<Game>>) -> Result<(), Error> {
    for (release_year, games) in clusters {
        let mut file = File::create(format!("{release_year}.txt"))?;
        for game in games {
            writeln!(file, "{}", game.to_string())?;
        }
    }
    Ok(())
}

pub fn save_producer_based_clusters(clusters: HashMap<String, Vec<Game>>) -> Result<(), Error> {
    for (producer, games) in clusters {
        let mut file = File::create(format!("{producer}.txt"))?;

        for game in games {
            writeln!(file, "{}", game.to_string())?;
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
        let clustered = cluster_by_release_year(games.unwrap());
        assert!(clustered.get(&1996).is_some());
    }

    #[test]
    fn get_cluster_by_producer_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let clustered = cluster_by_producer(games.unwrap());
        assert!(clustered.get("Nintendo").is_some());
    }

    #[test]
    fn save_producer_based_clusters_works_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let clustered = cluster_by_producer(games.unwrap());
        let result = save_producer_based_clusters(clustered);
        assert!(result.is_ok());
    }

    #[test]
    fn save_release_year_based_clusters_works_test() {
        let file_path = "game_data.txt";
        let games = read_file(file_path);
        let clustered = cluster_by_release_year(games.unwrap());
        let result = save_release_year_based_clusters(clustered);
        assert!(result.is_ok());
    }
}
