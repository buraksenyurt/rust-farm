use log::info;
use crate::common::sleep_while;

pub fn clear_home() -> bool {
    info!("Salonu temizlemeye başladım.");
    sleep_while(2.0);
    info!("Şu anda balkonu temizliyorum.");
    sleep_while(3.0);
    info!("Banyo da temizlendi");
    sleep_while(2.0);
    info!("Mutfakta bitmiştir");
    true
}
