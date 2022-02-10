use std::io;
use std::io::stdin;

fn main() {
    println!(
        "Bir gezegen seç.

        [1] - Merkür
        [2] - Venüs
        [3] - Mars
        [4] - Jüpiter
        [5] - Satürn
        [6] - Uranüs
        [7] - Neptün
        [8] - Ay"
    );

    // let mut planet = String::new();
    // io::stdin().read_line(&mut planet);

    let w = find_weight(60.0, Planet::Moon);
    println!("Dünyada 90 Kg Ayda {} kg'sın", w.unwrap());
}

#[allow(dead_code)]
enum Planet {
    Mercury,
    Mars,
    Venus,
    Saturn,
    Neptune,
    Jupiter,
    Uranus,
    Moon,
}

fn find_weight(w: f32, p: Planet) -> Option<f32> {
    match p {
        Planet::Mercury => Some((w / 9.81) * 3.7),
        Planet::Mars => Some((w / 9.81) * 3.711),
        Planet::Venus => Some((w / 9.81) * 8.87),
        Planet::Jupiter => Some((w / 9.81) * 24.79),
        Planet::Neptune => Some((w / 9.81) * 11.15),
        Planet::Uranus => Some((w / 9.81) * 8.69),
        Planet::Saturn => Some((w / 9.81) * 10.44),
        Planet::Moon => Some((w / 9.81) * 1.622),
    }
}
