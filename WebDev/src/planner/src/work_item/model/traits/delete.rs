use crate::{write_to_file, Storage};
use log::{error, info};
use serde_json::{Map, Value};

pub trait Delete {
    fn delete(&self, title: &str, state: &mut Map<String, Value>) {
        state.remove(title);
        let result = write_to_file(Storage::get().as_str(), state);
        match result {
            Ok(_) => info!("'{}' başlıklı görev silindi", title),
            Err(e) => error!("{}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::work_item::model::ready::Ready;
    use crate::work_item::model::traits::create::Create;
    use crate::work_item::model::traits::delete::Delete;
    use crate::work_item::model::traits::get::Get;
    use crate::work_item::status::Status;
    use serde_json::{Map, Value};

    #[test]
    fn should_delete_works() {
        let mut maps = Map::<String, Value>::new();

        let job = Ready::new("Odayı Temizle");
        job.create("Odayı Temizle", 8, Status::Doing, &mut maps);

        let title = "10bin Adım At";
        let job = Ready::new(title);
        job.create(title, 5, Status::Ready, &mut maps);

        job.delete(title, &mut maps);
        let actual = job.get(title, &maps);
        assert_eq!(actual, false);
    }
}
