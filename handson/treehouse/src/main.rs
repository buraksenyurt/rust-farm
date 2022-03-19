use treehouse::{get_player_name, get_visitors, Visitor};

fn main() {
    /*
       #1 Ekrandan girdi alıp gösterme işlemi yapıldı.
       #2 Ekrandan kullanıcı adını alan kod fonksiyonlaştırıldı.
       #3 Kullanıcının ağaç ev üyesi olup olmadığını anlayan fonksiyon ve testleri eklendi.
       #4 Ziyaretçiler için karşılama mesajlarını da tutan veri yapısı eklendi.
       #5 Ziyaretçi listesine yeni arkadaşlar eklenmesi sağlandı.
    */
    let mut visitors = get_visitors();
    loop {
        println!("Adın ne? Çıkmak için boş bırakıp Enter'a bas.");
        let player_name = get_player_name();
        let known_visitor = visitors.iter().find(|v| v.name == player_name);
        match known_visitor {
            Some(friend) => println!("{}", friend.say_hello()),
            None => {
                if player_name.is_empty() {
                    break;
                } else {
                    println!(
                        "Seni davetli listesinde bulamadım ama üzülme. Çünkü listeye eklendin :)"
                    );
                    visitors.push(Visitor::new(&player_name, "Yeni bir arkadaşımız var :)"))
                }
            }
        }
    }
    println!("Güncel ziyaretçi listemiz şöyle.\n{:#?}", visitors);
    // if is_tree_house_friend(&player_name) {
    //     println!("Hoş geldin, {}", player_name);
    // } else {
    //     println!("Üzgünüm '{}' ama seni tanımıyorum", player_name);
    // }
}
