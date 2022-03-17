use std::io::stdin;

fn main() {
    /*
       İlk kısımda ekrandan girdi alıp gösterme konusu işleniyor.
    */
    println!("Merhaba. Sana nasıl hitap etmemi istersin?");
    let mut player_name = String::new();
    stdin()
        .read_line(&mut player_name)
        .expect("Okuma işlemi sırasında hata");
    println!("Hoş geldin, {}", player_name.to_uppercase());
}
