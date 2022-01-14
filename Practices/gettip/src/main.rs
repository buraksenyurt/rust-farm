use std::env;

fn main() {
    // Terminalden girilen argümanları alalım
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let command = &args[1];
            println!("Girilen komut -> {}", command);
        }
        3 => {
            let category = &args[2];
            println!("`{}` ile ilgili bir ipucu aranacak", category);
        }
        _ => {
            println!("Lütfen kullanım klavuzunu okuyun");
        }
    };
}

fn load_tips() -> &'static Vec<Tip> {
    todo!()
}

fn get_random_tip(tips: &Vec<Tip>) -> String {
    todo!()
}

fn get_tips_by_category(tips: &Vec<Tip>, category: String) -> String {
    todo!()
}

struct Tip {
    id: i32,
    category: String,
    description: String,
}
