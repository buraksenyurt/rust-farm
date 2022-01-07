//use crate::warehouse::wh::load_players; // içeride tanımlı olan crate bildirimi
use crate::warehouse::wh::load_players as lp; // alias ile kullanım
use rand::Rng; // external crate bildirimi

mod warehouse;

fn main() {
    let country = "Brezilya";
    // load_players(country);
    lp(country);

    let mut randomizer = rand::thread_rng();
    let number: u32 = randomizer.gen();
    println!("Kazanan, {} numaralı oyuncu", number);

    println!("Bu numara çok anlamsız gibi. 1 ile 100 arası üretelim.");
    let number = randomizer.gen_range(1..100);
    println!("Ve şimdi kazanannnn, {} numaralı oyuncu.", number);

    println!("O da mı yok? Pof...Galiba oyuncular matematikçi. Tekrar deneyelim");
    let number = randomizer.gen_range(0.1..1.0);
    println!("Kazanan matematikçinin numarası {}", number * 10.0);
}
