use log::info;

pub trait Edit {
    fn set_to_doing(&self, title: &str) {
        info!("'{}' başlıklı görev statüsü Doing'e çekildi", title);
    }

    fn set_to_complete(&self, title: &str) {
        info!(
            "'{}' başlıklı görev statüsü Completed olarak değiştirildi",
            title
        );
    }
}
