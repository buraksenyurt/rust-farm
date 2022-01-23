use logging::Voyager;

fn main() {
    // önce loglayıcıyı oluşturalım
    env_logger::init();

    let mut gemini = Voyager::new(String::from("Gemini"));
    println!("{}\n", gemini.nickname);
    gemini.connect(String::from("Andromeda"));

    for _ in 0..3 {
        println!("{:?}", gemini);
        gemini.hited();
    }

    gemini.life = 1;
    gemini.connect(String::from("Orion"));
}
