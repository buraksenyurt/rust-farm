use std::io::{stdin, stdout, Write};
use std::process::Command;

/*
   single programındaki benzer şekilde terimalden gelen komutları işletir.
   Ancak farklı olarak argümanlarını da kullanır.

   ls -lah
   ps -ef
   cat notes.txt

   gibi komutları işletebilir
*/
fn main() {
    loop {
        print!("-> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Komut okunamadı");
        let cmd = input.trim();
        // boşluk karakterine göre gelen komutlar ayrıştırılır
        let arguments: Vec<&str> = cmd.split_whitespace().collect();
        // 0nc indis alt process komutudur. ls, cat gibi. 1.. ile de eklenen argümanlar bu process'e aktarılır.
        let mut child = Command::new(arguments[0])
            .args(&arguments[1..])
            .spawn()
            .expect("Komut işletilemiyor");
        child.wait().unwrap();
    }
}
