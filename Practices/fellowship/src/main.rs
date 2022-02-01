use crate::dursun::do_homework;
use crate::gibson::clear_home;
use crate::jhensen::do_shopping;
use log::{error, warn};
use std::thread;

mod common;
mod dursun;
mod gibson;
mod jhensen;

fn main() {
    env_logger::init();
    println!("Akşama misafir varrrr!!!");

    let market = vec![
        "Kuruyemiş",
        "Portakal Suyu",
        "8 Adet Muz",
        "2 Kilo Kızartmalık Patates",
    ];
    let mut handles = Vec::new();

    // İki tane thread başlatılıyoruz ve bunları handles'e ekliyoruz.
    // Nitekim ana thread'in bu iki thread'teki işler bitene kadar durmasını da sağlamalıyız.
    let jhensen_handle = thread::spawn(|| do_shopping(market));
    handles.push(jhensen_handle);
    let gibson_handle = thread::spawn(|| clear_home("Roventa Max"));
    handles.push(gibson_handle);

    // dursun'un işi ise main thread içinde çalışan normal bir fonksiyon
    do_homework("Lineer Cebir");

    // Yukarıda eş zamanlı başlatılan threar'lerin bitmesini beklettiğimiz yer
    for handle in handles {
        if handle.join().unwrap_or(false) {
            warn!("Bir iş bitti!");
        } else {
            error!("Upss. Bu işte bir yanlış var sanki");
        }
    }
    println!("Her şey yolunda. Misafirlerimizi bekliyoruz :)");
}
