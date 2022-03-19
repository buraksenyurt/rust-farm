use treehouse::{get_player_name, get_visitors, Visitor, VisitorAction};

fn main() {
    /*
       #1 Ekrandan girdi alıp gösterme işlemi yapıldı.
       #2 Ekrandan kullanıcı adını alan kod fonksiyonlaştırıldı.
       #3 Kullanıcının ağaç ev üyesi olup olmadığını anlayan fonksiyon ve testleri eklendi.
       #4 Ziyaretçiler için karşılama mesajlarını da tutan veri yapısı eklendi.
       #5 Ziyaretçi listesine yeni arkadaşlar eklenmesi sağlandı.
       #6 Ziyaretçilerin yaşı ve durumu(VisitorAction) için eklemeler yapıldı.
    */
    let mut visitors = get_visitors();
    loop {
        println!("Adın ne? Çıkmak için boş bırakıp Enter'a bas.");
        let player_name = get_player_name();
        let known_visitor = visitors
            .iter()
            .find(|v| v.name.to_lowercase() == player_name.to_lowercase());
        match known_visitor {
            Some(friend) => println!("{}", friend.say_hello()),
            None => {
                if player_name.is_empty() {
                    break;
                } else {
                    println!(
                        "Seni davetli listesinde bulamadım ama üzülme. Çünkü listeye eklendin :)"
                    );
                    visitors.push(Visitor::new(&player_name, VisitorAction::Candidate, 0))
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
