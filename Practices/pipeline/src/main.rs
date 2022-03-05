use std::io::{Read, Write};
use std::{env, io};

/*
   Program, terminal girdilerini 512 byte'lık bloklar haline okuyacak.
   Onu bir sabit olarak tanımlayabiliriz.
*/
const BLOCK_SIZE: usize = 512;

fn main() {
    // environment'ten bir parametre alıp nasıl kullandığımıza bakalım.
    let shadow_mode = env::var("SHADOW").unwrap_or_default() == "ON";
    let mut total_bytes = 0;
    loop {
        let mut buffer = [0; BLOCK_SIZE];
        let bytes_read = match io::stdin().read(&mut buffer) {
            Ok(0) => {
                println!("Gelen bir içerik yok");
                break;
            }
            Ok(length) => length,
            Err(e) => {
                println!("Bir hata oluştu.{}", e);
                break;
            }
        };
        total_bytes += bytes_read;
        match io::stdout().write_all(&buffer[..bytes_read]) {
            Ok(_) => println!("Yazma başarılı"),
            Err(e) => println!("Yazma işlemi sırasında hata. {}", e),
        };
    }
    if !shadow_mode {
        println!("{} byte kullanıldı.", total_bytes);
    }
}
