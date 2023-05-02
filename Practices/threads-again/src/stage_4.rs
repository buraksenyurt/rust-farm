/*
   multi producer single consumer kullanımı.
   Yani kanala(channel) mesaj bırakan çok sayıda thread olabilirken
   bu mesajları kanaldan alıp kullanacak tek bir consumer var olabilir.

   Birden fazla farklı değer için eş zamanlı hesaplamaların yapılacağı ama sonuçların bir merkezde
   örneğin main thread'de toplanacağı(aggregation) süreçlerde mpsc tercih edilebilir.

   Bu modelde thread'ler, kanala bıraktıkları değerlerin sahipliğini tekrardan alamazlar. Eğer
   gönderilen değerin korunması/gönderen thread tarafından tekrardan kullanılması istenen bir
   senaryo söz konusu ise shared-state concurrency modeli tercih edilmelidir.
*/
use std::sync::mpsc;
use std::thread;

fn main() {
    let (transmitter1, receiver) = mpsc::channel();
    let transmitter2 = mpsc::Sender::clone(&transmitter1);
    let transmitter3 = mpsc::Sender::clone(&transmitter1);

    thread::spawn(move || {
        let points = vec![1, 2, 3];
        for p in points {
            transmitter1.send(p).unwrap();
        }
    });

    thread::spawn(move || {
        let points = vec![4, 5, 6];
        for p in points {
            transmitter2.send(p).unwrap();
        }
    });

    thread::spawn(move || {
        let points = vec![7, 8, 9, 10];
        for p in points {
            transmitter3.send(p).unwrap();
        }
    });

    for rec_val in receiver {
        println!("Kanaldan gelen değer {}", rec_val);
    }
}
