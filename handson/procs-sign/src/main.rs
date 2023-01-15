use std::process;
use std::process::{Command};

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
        println!("İşlem başarısız");
    }

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
