use macromania::Snoopy;

#[derive(Snoopy)]
pub struct Player {
    id: i32,
    title: String,
}

fn main() {
    let _player = Player {
        id: 1,
        title: "Mario".to_string(),
    };
    Player::do_something();
}
