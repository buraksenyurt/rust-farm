use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let tips = load_tips();

    match args.len() {
        2 => {
            let command = &args[1];
            if command == "r" {
                println!("{}", get_random_tip(&tips));
            } else {
                println!("r girerek deneyin.");
            }
        }
        3 => {
            let category = &args[2];
            println!("`{}` ile ilgili bir ipucu aranacak", category);
        }
        _ => {
            println!("Rustgele bir ipucu için `r` ile\nBelli bir kategoride rustgele ipucu için `r rust` ile \ndeneyin lütfen;)");
        }
    };
}

fn load_tips() -> Vec<Tip> {
    let f = File::open("tips.json").expect("Dosya açılırken hata");
    let reader = BufReader::new(f);
    let tips: Vec<Tip> = serde_json::from_reader(reader).expect("json okumada hata");
    tips
}

fn get_random_tip(tips: &Vec<Tip>) -> String {
    let mut rng = thread_rng();
    let number = rng.gen_range(0..tips.len());
    tips[number].to_string()
}

fn get_tips_by_category(tips: &Vec<Tip>, category: String) -> String {
    todo!()
}

#[derive(Serialize, Deserialize)]
pub struct Tip {
    pub id: i32,
    pub category: String,
    pub description: String,
}

impl Display for Tip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.category, self.description)
    }
}
