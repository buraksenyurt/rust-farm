use logging::Voyager;

fn main() {
    let mut gemini = Voyager::new(String::from("Gemini"));
    println!("{}\n", gemini.nickname);
    gemini.connect(String::from("Andromeda"));

    for _ in 0..3 {
        println!("{:?}", gemini);
        gemini.hited();
    }
}
