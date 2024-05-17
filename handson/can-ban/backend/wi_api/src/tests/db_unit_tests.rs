#[cfg(test)]
mod tests {
    use crate::db::DbContext;
    use crate::models::WorkItem;
    use crate::types::{DurationType, Size, Status};
    use chrono::Local;

    #[test]
    fn test_create_work_item() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let new_item = WorkItem {
            id: 0,
            title: "Test Item".to_string(),
            duration: Some(120),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
        };

        let result = db.add_work_item(&new_item);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_work_item_status() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let new_item = WorkItem {
            id: 0,
            title: "Test Item".to_string(),
            duration: Some(120),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
        };

        let id = db.add_work_item(&new_item).unwrap();
        let payload = crate::api::UpdateStatusRequest {
            id,
            new_status: Status::Inprogress,
        };

        let result = db.update_work_item_status(&payload);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_to_archive_work_item() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let new_item = WorkItem {
            id: 0,
            title: "Test Item".to_string(),
            duration: Some(120),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
        };

        let id = db.add_work_item(&new_item).unwrap();
        let result = db.move_to_archive(id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_item_by_id() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let new_item = WorkItem {
            id: 0,
            title: "Test Item".to_string(),
            duration: Some(120),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
        };

        let id = db.add_work_item(&new_item).unwrap();
        let result = db.get_item(id);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.id, id);
        assert_eq!(item.title, "Test Item");
    }

    #[test]
    fn test_get_all_work_items() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let result = db.get_all();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_work_items_count() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let result = db.get_count();
        assert!(result.is_ok());
    }
}
