#[allow(dead_code)]

#[derive(Debug)]
struct Player {
    id: u32,
    nick: &str,
    country: &str,
    level: u16,
}

impl Player {
    fn new(id: u32, nick: &str, country: &str, level: u16) -> Self {
        Self {
            id,
            nick,
            country,
            level,
        }
    }
}

// #1nci Durum

// #[derive(Debug)]
// struct Player {
//     id: u32,
//     nick: String,
//     country: String,
//     level: u16,
// }

// impl Player {
//     fn new(id: u32, nick: String, country: String, level: u16) -> Self {
//         Self {
//             id,
//             nick,
//             country,
//             level,
//         }
//     }
// }


/*
    Fonksiyon bir Player değişkeninin referansını almakta ve ikinci parametre bilgisini kullanıp
    nickname'i değiştirdikten sonra geriye Player referansını iade etmekte.
 */
fn change_nickname(p: &mut Player, new_nickname: String) -> &Player {
    p.nick = new_nickname;
    p
}

fn main() {
    let mut gonzi = Player::new(1, "Gonsalez".to_string(), "Brasil".to_string(), 88);
    println!("{:#?}", gonzi);
    let ceremiya = change_nickname(&mut gonzi, "Ceremiya".to_string());
    println!("{:#?}", ceremiya);
}
