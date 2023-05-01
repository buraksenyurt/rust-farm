use std::io::{stdin, stdout, Write};
use std::process::Command;

/*
        Terminalden gelen komutu kullanarak yeni bir child process başlatır.
        sonsuz döngüde olduğundan kullanıcı sonlandırana kadar komut almaya devam eder.
 */
fn main() {
    loop {
        print!("-> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Girdi okuma hatası");
        let cmd = input.trim();
        let mut child = Command::new(cmd).spawn().expect("Komut işletilemiyor");
        child.wait().unwrap();
    }
}
