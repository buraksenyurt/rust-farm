pub mod entity {
    /*
    Serileştirilecek tiplerde Serialize ve Deserialize trait'lerini uygulanmış olması gerekir.
    derive bildirimlerinin sebebi budur.
    Game içinde kullanılan Kind enum türünün de serileşebilir olması gerekir.
     */
    use serde::{Deserialize, Serialize};
    use std::fmt::{Display, Formatter};

    #[derive(Serialize, Deserialize)]
    pub struct Game {
        pub id: i32,
        pub title: String,
        pub kind: Kind,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Kind {
        Adventure,
        Fps,
        Rpg,
        Rts,
    }

    impl Display for Game {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}-{}({:?})", self.id, self.title, self.kind)
        }
    }
}
