fn main() {
    let mut calderon = Player {
        id: 1,
        name: String::from("Hoze Kalderon"),
        level: 78,
    };
    increase_level(&mut calderon);
    dbg!(calderon.level);
    decrease_level(&mut calderon);
    dbg!(calderon.level);
}

fn increase_level(p: &mut Player) {
    p.level += 10;
}
fn decrease_level(p: &mut Player) {
    let rate = 10;
    p.level -= rate;
}

#[allow(dead_code)]
struct Player {
    id: u16,
    name: String,
    level: u16,
}