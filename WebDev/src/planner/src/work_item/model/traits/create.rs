use log::info;

pub trait Create {
    fn create(&self, title: &str, value: u16) {
        info!(
            "'{}' başlıklı ve {} değerindeki görev oluşturuldu",
            title, value
        );
    }
}
