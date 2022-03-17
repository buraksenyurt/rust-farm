use crate::work_item::mission::Mission;
use crate::work_item::model::completed::Completed;
use crate::work_item::model::doing::Doing;
use crate::work_item::model::ready::Ready;
use crate::work_item::size::Size;
use crate::work_item::status::Status;

/// Görev ile ilgili fabrika nesnesidir.
pub struct Factory {}

impl Factory {
    /// Belli bir statüye göre bir görev oluşturur
    pub fn create_work_item(wi_status: Status, wi_title: &str, wi_size: Size) -> Option<Mission> {
        match wi_status {
            Status::Ready => Some(Mission::Ready(Ready::new(wi_title, wi_size))),
            Status::Doing => Some(Mission::Doing(Doing::new(wi_title, wi_size))),
            Status::Completed => Some(Mission::Completed(Completed::new(wi_title, wi_size))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::mission::Mission;
    use super::*;

    #[test]
    fn should_factory_works() {
        let job = Factory::create_work_item(Status::Ready, "Odayı temizleme görevi", Size::Large)
            .unwrap();
        let expected = Mission::Ready(Ready::new("Odayı temizleme görevi", Size::Large));
        assert_eq!(job, expected);

        let job = Factory::create_work_item(Status::Doing, "Odayı temizleme görevi", Size::Large)
            .unwrap();
        let expected = Mission::Doing(Doing::new("Odayı temizleme görevi", Size::Large));
        assert_eq!(job, expected);

        let job =
            Factory::create_work_item(Status::Completed, "Odayı temizleme görevi", Size::Large)
                .unwrap();
        let expected = Mission::Completed(Completed::new("Odayı temizleme görevi", Size::Large));
        assert_eq!(job, expected);
    }
}
