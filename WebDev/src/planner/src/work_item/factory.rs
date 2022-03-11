use crate::work_item::mission::Mission;
use crate::work_item::model::completed::Completed;
use crate::work_item::model::doing::Doing;
use crate::work_item::model::ready::Ready;
use crate::work_item::status::Status;

/// Görev ile ilgili fabrika nesnesidir.
pub struct Factory {}

impl Factory {
    /// Belli bir statüye göre bir görev oluşturur
    pub fn create_work_item(wi_status: Status, wi_title: &str, wi_value: u16) -> Option<Mission> {
        match wi_status {
            Status::Ready => Some(Mission::Ready(Ready::new(wi_title, wi_value))),
            Status::Doing => Some(Mission::Doing(Doing::new(wi_title, wi_value))),
            Status::Completed => Some(Mission::Completed(Completed::new(wi_title, wi_value))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::mission::Mission;
    use super::*;

    #[test]
    fn should_factory_works() {
        let job = Factory::create_work_item(Status::Ready, "Odayı temizleme görevi", 5).unwrap();
        let expected = Mission::Ready(Ready::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);

        let job = Factory::create_work_item(Status::Doing, "Odayı temizleme görevi", 5).unwrap();
        let expected = Mission::Doing(Doing::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);

        let job =
            Factory::create_work_item(Status::Completed, "Odayı temizleme görevi", 5).unwrap();
        let expected = Mission::Completed(Completed::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);
    }
}
