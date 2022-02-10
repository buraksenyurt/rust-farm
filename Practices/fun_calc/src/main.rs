use std::io;

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

    let mut planet = String::new();
    io::stdin()
        .read_line(&mut planet)
        .expect("Developer hatası :P");

    println!(
        "Şimdi de bir ağırlık gir.\nKendi kilon, bilgisayarının ki, oyuncak otomobilinin ki\nhatta bir balinanın ki bile olabilir.(kg)"
    );

    let mut input_weight = String::new();
    io::stdin()
        .read_line(&mut input_weight)
        .expect("Developer hatası :P");

    let weight: f32 = match input_weight.trim().parse::<f32>() {
        Ok(w) => match planet.trim().to_lowercase().as_str() {
            "1" => find_weight(w, Planet::Mercury).unwrap(),
            "2" => find_weight(w, Planet::Venus).unwrap(),
            "3" => find_weight(w, Planet::Mars).unwrap(),
            "4" => find_weight(w, Planet::Jupiter).unwrap(),
            "5" => find_weight(w, Planet::Saturn).unwrap(),
            "6" => find_weight(w, Planet::Uranus).unwrap(),
            "7" => find_weight(w, Planet::Neptune).unwrap(),
            "8" => find_weight(w, Planet::Moon).unwrap(),
            _ => {
                println!("Seçimin yanlış olabilir mi?");
                0.0
            }
        },
        Err(e) => {
            println!("Sanırım girdiğin kilo sayısal bir değer değil.\n{:?}", e);
            0.0
        }
    };

    println!("Dünyada {} Kg seçtiğin gezegende ise {} kg'sın",&input_weight.trim(), weight);
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
