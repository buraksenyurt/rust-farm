use std::env;

fn main() {
    // Terminalden girilen argümanları alalım
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let command = &args[1];
            println!("Girilen komut -> {}",command);
        }
        3 => {
            let category = &args[2];
            println!("`{}` ile ilgili bir ipucu aranacak", category);
        }
        _ => {
            println!("Lütfen kullanım klavuzunu okuyun");
        }
    };
}
