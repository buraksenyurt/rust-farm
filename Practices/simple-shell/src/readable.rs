use std::io::{stdin, stdout, Write};
use std::process::Command;

/*
   list files şeklinde gelen komutu alıp ls komutu olarak çalıştırır
*/
fn main() {
    loop {
        print!("-> ");
        stdout().flush().unwrap();
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Girdi okunamadı");
        let cmd = input.trim();
        let arguments: Vec<&str> = cmd.split_whitespace().collect();
        if arguments.len() > 0 {
            match arguments[0] {
                "list" if arguments.len() > 1 => match arguments[1] {
                    "files" => {
                        handle_child(&arguments);
                    }
                    _ => println!("Geçersiz komut"),
                },
                "exit" => std::process::exit(0),
                _ => {
                    handle_child(&arguments);
                }
            }
        }
    }
}

fn handle_child(arguments: &Vec<&str>) {
    let child = Command::new("ls").args(&arguments[2..]).spawn();
    match child {
        Ok(mut c) => {
            if c.wait().unwrap().success() {
            } else {
                println!("İşlem hatası")
            }
        }
        Err(e) => println!("{}", e),
    };
}
