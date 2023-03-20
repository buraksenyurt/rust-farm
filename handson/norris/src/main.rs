use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::net::SocketAddr;
use std::path::Path;

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    let route = Router::new().route("/norris", get(default_handler));
    let app = Router::new().nest("/api", route).with_state(app_state);

    let address = SocketAddr::from(([127, 0, 0, 1], 6001));
    println!(
        "Sunucu başlatılıyor: {}.\nÇek bir Chuck Norris sözü.",
        address
    );

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn default_handler(State(app_state): State<AppState>) -> Json<Fact> {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..app_state.get().len()) as usize;
    if let Some(f) = app_state.get().get(index) {
        return Json(f.clone());
    }
    Json(Fact::new(0, "May be another time".to_string()))
}

fn load_facts() -> Option<Vec<Fact>> {
    println!("Norris facts are loading");
    if let Ok(lines) = read_lines("./norris_facts.dat") {
        let mut facts: Vec<Fact> = Vec::new();
        for (i, line) in lines.enumerate() {
            if let Ok(l) = line {
                facts.push(Fact::new(i as u32, l));
            }
        }
        return Some(facts);
    }
    None
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(serde::Serialize, Clone)]
struct Fact {
    id: u32,
    content: String,
}

impl Fact {
    pub fn new(id: u32, content: String) -> Self {
        Self { id, content }
    }
}

#[derive(Clone)]
struct AppState {
    facts: Vec<Fact>,
}

impl AppState {
    pub fn new() -> Self {
        let facts = load_facts().unwrap();
        Self { facts }
    }
    pub fn get(&self) -> &Vec<Fact> {
        &self.facts
    }
}

#[cfg(test)]
mod test {
    use crate::load_facts;

    #[test]
    fn read_file_lines_test() {
        let facts = load_facts();
        assert!(facts.is_some());
        assert!(facts.unwrap().len() > 0);
    }
}
