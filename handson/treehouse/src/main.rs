use treehouse::{get_player_name, get_visitors};

fn main() {
    /*
       #1 Ekrandan girdi alıp gösterme işlemi yapıldı.
       #2 Ekrandan kullanıcı adını alan kod fonksiyonlaştırıldı.
       #3 Kullanıcının ağaç ev üyesi olup olmadığını anlayan fonksiyon ve testleri eklendi.
       #4 Ziyaretçiler için karşılama mesajlarını da tutan veri yapısı eklendi.
    */
    println!("Merhaba. Sana nasıl hitap etmemi istersin?");
    let player_name = get_player_name();
    let visitors = get_visitors();
    let known_visitor = visitors.iter().find(|v| v.name == player_name);
    match known_visitor {
        Some(friend) => println!("{}", friend.say_hello()),
        None => println!("Seni davetli listesinde bulamadım :|"),
    }
    // if is_tree_house_friend(&player_name) {
    //     println!("Hoş geldin, {}", player_name);
    // } else {
    //     println!("Üzgünüm '{}' ama seni tanımıyorum", player_name);
    // }
}
