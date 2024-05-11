#[cfg(test)]
mod test {
    use crate::model::{DurationType, Size, Status, WorkItem};
    use chrono::Local;

    #[test]
    pub fn enums_returns_number_test() {
        let work_item = WorkItem {
            id: 1,
            title: "Give your old magazines to best friends".to_string(),
            size: Some(Size::Medium),
            duration: Some(1),
            duration_type: Some(DurationType::Hour),
            status: Status::Completed,
            crate_date: Local::now(),
            modified_date: None,
        };

        let d_type = work_item.duration_type.unwrap() as u8;
        assert_eq!(d_type, 1);
        let s = work_item.status as u8;
        assert_eq!(s, 3);
        let size = work_item.size.unwrap() as u8;
        assert_eq!(size, 2);
    }

    #[test]
    pub fn numbers_to_enums_test() {
        assert_eq!(Size::Small, Size::try_from(1_u8).unwrap());
        assert_eq!(DurationType::Week, DurationType::try_from(3_u8).unwrap());
        assert_eq!(Status::Inprogress, Status::try_from(2_u8).unwrap());
    }
}
