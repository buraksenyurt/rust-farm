use log::info;
use serde_json::{Map, Value};

pub trait Get {
    fn get(&self, title: &str, state: &Map<String, Value>) -> bool {
        let wi = state.get(title);
        match wi {
            Some(item) => {
                info!("{}\n\n{}", title, item);
                true
            }
            None => {
                info!("{} başlıklı bir görev bulunamadı", title);
                false
            }
        }
    }
}
