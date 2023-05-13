use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
    // Standart dosyaya yazma örneği

    // Dosya oluşturulur. Not: Program kodu her çalıştığında yeni bir scneario.txt oluşur
    let f = File::create("scenario.txt").unwrap();
    // Bu dosyayı kullanacak BufWriter örneklenir
    let mut buf_writer = BufWriter::new(f);
    // Dosya içeriğine yazılacak bir String içerik hazırlanır
    let buffer = "Oyun kahramanımızın sahilde uyanması ile başlar".to_string();
    // BufWriter ile bu içerik temsil ettiği hedefe(dosya) yazılır
    buf_writer.write_all(buffer.as_bytes()).unwrap();
    println!("Dosyaya yazılan bilgi aşağıdaki gibidir.\n{}", buffer);
}
