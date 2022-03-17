use treehouse::{get_player_name, is_tree_house_friend};

fn main() {
    /*
       #1 Ekrandan girdi alıp gösterme işlemi yapıldı.
       #2 Ekrandan kullanıcı adını alan kod fonksiyonlaştırıldı.
    */
    println!("Merhaba. Sana nasıl hitap etmemi istersin?");
    let player_name = get_player_name();
    if is_tree_house_friend(&player_name) {
        println!("Hoş geldin, {}", player_name);
    } else {
        println!("Üzgünüm '{}' ama seni tanımıyorum", player_name);
    }
}
