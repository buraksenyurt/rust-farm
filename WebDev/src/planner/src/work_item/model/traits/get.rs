use log::info;

pub trait Get {
    fn get(&self, title: &str) {
        info!("{} başlıklı görev çekildi", title);
    }
}
