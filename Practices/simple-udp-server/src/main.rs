mod message;

use crate::message::Message;
use std::net::UdpSocket;
use std::thread;

fn main() {
    println!("UDP Sunucu Aktif...Çıkmak için CTRL+C");

    // İstemciden gelen mesajlar için açılan thread'leri senkronize etmek amacıyla eklendi
    let mut threads = Vec::new();

    // Öncelikle bir Udp Socket oluşturulur. Senaryoda localhost:5001 nolu porttan ayağa kalkar.
    let socket = UdpSocket::bind("127.0.0.1:5001").expect("UDP porta bağlanılamadı.");
    // Taşınacak datagram'lar için 1 Kb'lık bir buffer nesnesi tanımlanır
    let mut buffer = [0; 1024];
    // Sunucu sürekli dinlemede olacağından sonsuz bir döngü gerekir
    loop {
        // İstemci taleplerinde soketi kullanabilmek için bir klonu oluşturulur
        let socket_c = socket.try_clone().expect("Soket nesnesi kopyalanamadı.");
        // Eğer sokete gelen bir datagram bilgisi varsa match Ok dalı ile ilerler
        match socket_c.recv_from(&mut buffer) {
            // Gelen bilginin uzunluğu ve bağlanan istemci adresi yakalanabilir
            Ok((len, src)) => {
                println!("Veri uzunluğu {}. İstemci adresi {}", len, src);
                // Gelen bilgiyi işleyecek ayrı bir thread başlatılır
                let thread = thread::spawn(move || {
                    // Gelen datagram uzunluğuna göre bir buffer daha ayarlanır
                    let input = &mut buffer[..len];

                    let decoded_input: Message =
                        bincode::deserialize(input).expect("Gelen mesaj çözümlenemiyor");

                    // Bilgi test amaçlı sunucu ekranına yazdırılır
                    println!(
                        "İstemciden gelen bilgi:\n\tID:{}\n\tMessage:{}",
                        decoded_input.id, decoded_input.content
                    );
                    // // İstemci tarafa döndürülmek üzere bir çıktı mesajı hazırlanır
                    // // from_utf8_lossy byte array'i String'e dönüştürürken geçersiz karakterleri
                    // // de işin içerisine katar.
                    // let output = format!(
                    //     "Mesajın alındı. Bana şu mesajı göndermiştin, '{}'",
                    //     String::from_utf8_lossy(input)
                    // );
                    let output = format!("{} numaralı mesajın alındı", decoded_input.id);

                    // Soket nesnesinin send_to fonksiyonundan yararlanılarak
                    // istemci adresine datagram bilgisi dönülür
                    socket_c
                        .send_to(output.as_bytes(), src)
                        .expect("Datagram gönderimi sırasında hata oluştu.");
                });

                threads.push(thread);
            }
            Err(e) => {
                println!("'{}' nedeniyle datagram alınamıyor.", e)
            }
        }

        // İstemciden gelen mesajlar için açılan thread'leri senkronize etmek amacıyla eklendi
        // Böylece mesajlar karışık değil sıralı şekilde işlenir.
        for thread in threads.drain(..) {
            thread.join().expect("Thread tamamlanamadı.");
        }
    }
}
