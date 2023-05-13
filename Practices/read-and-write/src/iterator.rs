use std::fs::File;
use std::io::{stdin, BufRead, BufReader};

fn main() {
    /*
       Rust std::io modülündeki pek çok veri modelinin
       işleri kolaylaştıran iterator'leri vardır.
    */
    println!("çıkmak için 'exit' yaz");
    // Standard input stream nesnesi örneklenir
    // console uygulaması olduğundan terminalden girdi bekler
    let input = stdin();
    // girdiyi okuyacak bir Buffer Reader örneklenir
    let reader = BufReader::new(input);
    // satır girildikçe ekrana basılır.
    for line in reader.lines() {
        let l = line.unwrap();
        if l.to_lowercase() == "exit" {
            break;
        } else {
            println!("\t{}", l);
        }
    }

    // Örneğin dosya içeriklerini okurken de iterator'lerden yararlanılır
    let f = File::open("data.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
