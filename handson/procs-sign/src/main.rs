/*
   Practical System Programming for Rust Developers kitabından çalıştığım
   Working with Process and Signals bölümüne ait kodlar ve açıklamaları.
*/
use std::io::{Read, Write};
use std::process;
use std::process::{Command, Stdio};

fn main() {
    // // İşletim sisteminden bir programı process olarak açmak için
    // // Örnekte ls terminal programı l ve h argümanları ile birlikte çalıştırılmakta.
    // Command::new("ls")
    //     .arg("-l")
    //     .arg("-h")
    //     .spawn()
    //     .expect("ls komutu çalıştırılamadı");
    //
    // // yukarıdaki komut aşağıdaki gibi de çalıştırılabilir.
    // Command::new("ls")
    //     .args(["-l", "-h"])
    //     .spawn()
    //     .expect("ls komutu çalıştırılamadı");

    // aşağıdaki kod parçasında process'i ele alabileceğimiz nesne(handler) kullanımı söz konusudur.

    let ls_handler = Command::new("ls").args(["-l", "-h"]).output().unwrap();
    if ls_handler.status.success() {
        println!(
            "Klasör içeriği \n{}",
            String::from_utf8(ls_handler.stdout).unwrap()
        );
    }

    let cat_handler = Command::new("cat").arg("data.txt").output().unwrap();
    if cat_handler.status.success() {
        println!(
            "Data.txt içeriği \n{}",
            String::from_utf8(cat_handler.stdout).unwrap()
        );
    }

    // Başlatılan bir process'in durumunu(status) aşağıdaki şekilde de kontrol edebiliriz.
    let cat_status = Command::new("cat")
        .arg("there-is-no-file.txt")
        .status()
        .expect("cat komutunun çalıştırılmasında hata");
    if cat_status.success() {
        println!("İşlem başarılı");
    } else {
        println!("cat there-is-no-file.txt komutu başarısız");
    }

    // Bir Parent Process ile başlattığı Child Process arasında iletişim hattı(pipe) açıp
    // process çıktılarını ele alabiliriz.
    // ps komutu ile sistemde çalışan process'lere ait bilgileri görüntüleyebiliriz.
    // Aşağıdaki kod parçasında ps için bir child process açılır ve çıktısı açılan pipe aracılığı
    // ile process_output isimli bir String değişkene alınır.
    let process = match Command::new("ps").arg("a").stdout(Stdio::piped()).spawn() {
        Ok(process) => process,
        Err(e) => panic!("ps komutu işletilemedi. {}", e),
    };
    let mut process_output = String::new();
    match process.stdout.unwrap().read_to_string(&mut process_output) {
        Ok(_) => println!("Process Listesi\n{}", &process_output),
        Err(e) => println!("ps komutunun çıktısı okunamadı. {}", e),
    };

    // Aşağıdaki kod parçasında ise parent process'ten child process'e veri gönderimi
    // ele alınmaktadır. Bunu deneyimlemek için bir metin katarını tersine döndüren rev komutu
    // kullanılmakta.

    // önce process handler oluşturulur.
    // Bu yapılırken child process ile piped türünden hem girdi hem de çıktı için
    // çift yönlü hat tesis edilir.
    // rev komutuna dikkat edileceği üzere bir argüman henüz gönderilmedi.
    let rev_process = match Command::new("rev")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Ok(prc) => prc,
        Err(e) => panic!("rev komutu oluşturulurken hata. {}", e),
    };

    // process handler'in girdi hattı kullanılarak
    // rev komutunun çalıştırması istenen bilgi gönderilir
    match &rev_process
        .stdin
        .unwrap()
        .write_all("Hello from New England".as_bytes())
    {
        Ok(_) => println!("rev komutuna bilgi gönderildi"),
        Err(e) => println!("rev komutuna arguman gönderilemedi {}", e),
    };

    // alt process'in işlettiği komutun çıktısını almak içinde stdout kullanılır.
    let mut child_output = String::new();
    match &rev_process
        .stdout
        .unwrap()
        .read_to_string(&mut child_output)
    {
        Ok(_) => println!("\t{}", child_output),
        Err(e) => println!("rev işlem sonucu alınamadı.{}", e),
    };

    // // Bir process'si sonlandırmak için iki yöntem var.
    // // abort ve exit.
    // process::abort();
    // println!("Abort nedeniyle bu satır işletimez");

    // exit fonksiyonu ile process'den geriye hata kodu da dönülebilir.
    // Unix tabanlı sistemlerde 0-255 arası standart hata kodları mevcuttur.
    // 0 kodu Success anlamına gelir. Bir sürecin içinde oluşan hatalar başka süreçler için
    // önem arz edebilir. Bu nedenle duruma göre exit fonksiyonu tercih edilmelidir.
    // https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+11.2-stable&arch=default&format=html
    process::exit(64);
}
