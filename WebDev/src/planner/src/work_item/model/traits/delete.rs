use log::info;

pub trait Delete {
    fn delete(&self, title: &str) {
        info!("'{}' başlıklı görev silindi", title)
    }
}
