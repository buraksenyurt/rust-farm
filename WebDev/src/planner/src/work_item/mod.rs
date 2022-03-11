use model::completed::Completed;
use model::doing::Doing;
use model::ready::Ready;

mod model;

pub enum Status {
    Ready,
    Doing,
    Completed,
}

#[derive(Debug, PartialEq)]
pub enum WorkItem {
    Ready(Ready),
    Doing(Doing),
    Completed(Completed),
}

pub fn work_item_factory(wi_status: Status, wi_title: &str, wi_value: u16) -> Option<WorkItem> {
    match wi_status {
        Status::Ready => Some(WorkItem::Ready(Ready::new(wi_title, wi_value))),
        Status::Doing => Some(WorkItem::Doing(Doing::new(wi_title, wi_value))),
        Status::Completed => Some(WorkItem::Completed(Completed::new(wi_title, wi_value))),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_factory_works() {
        let job = work_item_factory(Status::Ready, "Odayı temizleme görevi", 5).unwrap();
        let expected = WorkItem::Ready(Ready::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);

        let job = work_item_factory(Status::Doing, "Odayı temizleme görevi", 5).unwrap();
        let expected = WorkItem::Doing(Doing::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);

        let job = work_item_factory(Status::Completed, "Odayı temizleme görevi", 5).unwrap();
        let expected = WorkItem::Completed(Completed::new("Odayı temizleme görevi", 5));
        assert_eq!(job, expected);
    }
}
