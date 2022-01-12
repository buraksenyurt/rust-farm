pub mod entity {
    use std::fmt::{Display, Formatter};

    pub struct Game {
        pub id: i32,
        pub title: String,
        pub kind: Kind,
    }

    #[derive(Debug)]
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
