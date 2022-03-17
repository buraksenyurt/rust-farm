use std::io::stdin;

pub fn get_player_name() -> String {
    let mut player_name = String::new();
    stdin()
        .read_line(&mut player_name)
        .expect("Okuma işlemin hata. Programcını ara");
    player_name.trim().to_lowercase()
}
