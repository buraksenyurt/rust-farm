use crate::work_item::status::Status;
use crate::{write_to_file, Storage};
use log::{error, info};
use serde_json::{json, Map, Value};

pub trait Edit {
    fn set_to_doing(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(Status::Doing.to_string()));
        let result = write_to_file(Storage::get().as_str(), state);
        match result {
            Ok(_) => info!("'{}' başlıklı görev statüsü Doing'e çekildi", title),
            Err(e) => error!("{}", e),
        }
    }

    fn set_to_complete(&self, title: &str, state: &mut Map<String, Value>) {
        state.insert(title.to_string(), json!(Status::Completed.to_string()));
        let result = write_to_file(Storage::get().as_str(), state);
        match result {
            Ok(_) => info!(
                "'{}' başlıklı görev statüsü Completed olarak değiştirildi",
                title
            ),
            Err(e) => error!("{}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::work_item::model::ready::Ready;
    use crate::work_item::model::traits::edit::Edit;
    use crate::work_item::status::Status;
    use serde_json::{json, Map, Value};

    #[test]
    fn should_edit_works() {
        let title = "Rust çalış";
        let job = Ready::new(title);
        let mut maps = Map::<String, Value>::new();
        maps.insert(title.to_string(), json!(Status::Ready.to_string()));
        job.set_to_doing(title, &mut maps);
        let actual = maps.get(title);
        let expected = json!(Status::Doing.to_string());
        assert_eq!(actual, Some(&expected));
    }
}
