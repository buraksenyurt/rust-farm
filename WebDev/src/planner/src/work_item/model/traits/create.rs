use crate::work_item::size::Size;
use crate::work_item::status::Status;
use crate::{write_to_file, Storage};
use log::{error, info};
use serde_json::{json, Map, Value};

pub trait Create {
    fn create(&self, title: &str, size: &Size, status: Status, state: &mut Map<String, Value>) {
        let v = json!({ "value": size,"state": status.to_string() });
        state.insert(title.to_string(), v);
        let result = write_to_file(Storage::get().as_str(), state);
        match result {
            Ok(_) => info!(
                "'{}' başlıklı ve {} büyüklüğündeki görev {} statüsünde oluşturuldu",
                title, size, status
            ),
            Err(e) => error!("{}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::work_item::model::ready::Ready;
    use crate::work_item::model::traits::create::Create;
    use crate::work_item::model::traits::get::Get;
    use crate::work_item::size::Size;
    use crate::work_item::status::Status;
    use serde_json::{json, Map, Value};

    #[test]
    fn should_create_works() {
        let job = Ready::new("Odayı Temizle", Size::Large);
        let mut maps = Map::<String, Value>::new();
        job.create("Odayı Temizle", &Size::Large, Status::Doing, &mut maps);
        let actual = job.get("Odayı Temizle", &maps);
        let expected = json!({ "value": Size::Large,"state": "Doing" });
        assert_eq!(actual, Some(expected));
    }
}
