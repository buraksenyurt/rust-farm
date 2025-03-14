use std::io::{stdin, BufRead};

/*
     echo "hello world" "how are you?" "I am good and you?" | cargo run

     veya

     cargo build --release sonrası doğrudan

     echo "hello world" "how are you?" "I am good and you?" | cpul_reader

     şeklinde denenebilir.
*/
fn main() {
    let stdin = stdin();
    let lock = stdin.lock();

    for line in lock.lines() {
        match line {
            Ok(line) => {
                println!("{}", line);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}
