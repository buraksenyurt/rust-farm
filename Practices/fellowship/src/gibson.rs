use crate::common::sleep_while;
use log::info;

pub fn clear_home(equipment: &str) -> bool {
    info!("Salonu temizlemeye başladım. Malzeme {}", equipment);
    sleep_while(2.0);
    info!("Şu anda balkonu temizliyorum.");
    sleep_while(3.0);
    info!("Banyo da temizlendi");
    sleep_while(2.0);
    info!("Mutfakta bitmiştir");
    true
}
