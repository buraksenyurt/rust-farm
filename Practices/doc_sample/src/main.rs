use doc_sample::{Level, Model};

fn main() {
    let mut m109 = Model::new(String::from("Meserrschmitt 109"), Level::Easy, 42, 270.50);
    println!("{}", m109.to_string());
    m109.apply_discount(32.0);
    println!("{}", m109.to_string());
}
