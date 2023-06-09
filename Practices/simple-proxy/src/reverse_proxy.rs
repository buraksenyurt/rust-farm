use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::{env, thread};

fn main() {
    // Terminalden proxy ve backend sunucularının adreslerini toplayacağız
    // cargo run --bin reverse_proxy localhost:5555 localhost:3002
    // gibi
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Lütfen proxy ve backend adreslerini girin");
        exit(2);
    }
    let proxy_server = &args[1];
    let backend_server = &args[2];
    let listener;
    // proxy sunucu adresine bağlantı sağlanabiliyorsa
    if let Ok(proxy) = TcpListener::bind(proxy_server) {
        listener = proxy;
        let address = listener.local_addr().unwrap().ip();
        let port = listener.local_addr().unwrap().port();
        // origin server(backend server) a bir bağlantı denenir
        if let Err(e) = TcpStream::connect(backend_server) {
            eprintln!("Backend server'a bağlanılamıyor. {}", e);
            exit(1);
        }
        println!("{}:{} çalışıyor\n", address, port);
    } else {
        eprintln!("Proxy sunucu bağlantısında hata");
        exit(1);
    }

    // Açılan tüm thread'ler için bir kuyruk tanımlanır
    let mut threads = Vec::new();
    // proxy server'a gelen istekler dinlenmeye başlar
    for stream in listener.incoming() {
        let mut proxy_stream = stream.expect("TCP bağlantısında hata");
        // bu arada backend server ile de bir bağlantı başlatılır
        // gelen her istek için bir bağlantı tesis edilir
        let mut backend_stream =
            TcpStream::connect(backend_server).expect("Backend server erişim sorunu.");
        // Birden fazla talep gelebileceğinden bunlar ayrı thread'lerce ele alınır
        let t = thread::spawn(move || handle_stream(&mut proxy_stream, &mut backend_stream));
        threads.push(t);
    }
    // Ne kadar thread varsa tamamı işini bitirene kadar beklenilmesi sağlanır
    for t in threads {
        t.join().expect("Thread hatası");
    }
}

// proxy ve backend arasındaki akışı kontrol eden fonksiyon
fn handle_stream(proxy_stream: &mut TcpStream, backend_stream: &mut TcpStream) {
    // Girdi ve Çıktılar için buffer vector'ler tahsis edilir
    let mut input_buffer: Vec<u8> = vec![0; 1024];
    let mut output_buffer: Vec<u8> = vec![0; 1024];

    if let Err(e) = proxy_stream.read(&mut input_buffer) {
        eprintln!(
            "Proxy'den gelen bilginin okunması sırasında hata oluştu.{}",
            e
        );
    } else {
        // Gelen istek bilgi amaçlı ekrana bastırılır
        println!("Gelen istek -> {}", String::from_utf8_lossy(&input_buffer));
    }

    // İstek backend sunucuya açılan stream'e yazılır
    let _ = backend_stream.write(&mut input_buffer).unwrap();
    // buna karşın backend taraftan gelen cevap'ta output'a yazılır
    let _ = backend_stream.read(&mut output_buffer).unwrap();
    println!(
        "Backend server'dan gelen cevap -> {}",
        String::from_utf8_lossy(&output_buffer)
    );
    // Son olarak proxy'ye gelmiş olan cevap istemciyle arada açılan stream'a aktarılır
    let _ = proxy_stream.write(&mut output_buffer).unwrap();
}
