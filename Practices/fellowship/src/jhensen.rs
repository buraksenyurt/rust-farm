use crate::common::sleep_while;
use log::info;

pub fn do_shopping(list: Vec<&str>) -> bool {
    info!("Alışveriş listesini aldım. Göreve başlıyorum.\n{:#?}", list);
    // sembolik olarak bu thread'i 5 saniye duraksatıyoruz
    sleep_while(5.0);
    info!("Alışveriş tamamlandı ve eve geldim :)");
    true
}
