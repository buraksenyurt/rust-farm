#[cfg(test)]
mod tests {
    use crate::db::DbContext;
    use crate::models::WorkItem;
    use chrono::Local;
    use shared::*;

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
            finish_date: None,
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
            finish_date: None,
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
            finish_date: None,
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
            finish_date: None,
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
        let result = db.get_all(false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_work_items_count() {
        let db = DbContext::new(true).expect("Failed to initialize database");
        let result = db.get_count();
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_summary() {
        let db = DbContext::new(true).expect("Failed to initialize database"); // Use in-memory db
        let new_item_1 = WorkItem {
            id: 0,
            title: "Test Item 1".to_string(),
            duration: Some(1),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Todo,
            crate_date: Local::now(),
            modified_date: None,
            finish_date: None,
        };

        let new_item_2 = WorkItem {
            id: 0,
            title: "Test Item 2".to_string(),
            duration: Some(1),
            duration_type: Some(DurationType::Hour),
            size: Some(Size::Small),
            status: Status::Inprogress,
            crate_date: Local::now(),
            modified_date: None,
            finish_date: None,
        };

        let new_item_3 = WorkItem {
            id: 0,
            title: "Test Item 3".to_string(),
            duration: Some(3),
            duration_type: Some(DurationType::Day),
            size: Some(Size::Epic),
            status: Status::Completed,
            crate_date: Local::now(),
            modified_date: Some(Local::now()),
            finish_date: None,
        };

        let new_item_4 = WorkItem {
            id: 0,
            title: "Test Item 4".to_string(),
            duration: Some(2),
            duration_type: Some(DurationType::Day),
            size: Some(Size::Epic),
            status: Status::Completed,
            crate_date: Local::now(),
            modified_date: Some(Local::now()),
            finish_date: None,
        };

        db.add_work_item(&new_item_1).unwrap();
        db.add_work_item(&new_item_2).unwrap();
        db.add_work_item(&new_item_3).unwrap();
        db.add_work_item(&new_item_4).unwrap();

        let summary_report = db.get_summary_report().unwrap();

        assert_eq!(summary_report.work_items, 4);
        assert_eq!(summary_report.todo_items, 1);
        assert_eq!(summary_report.in_progress_items, 1);
        assert_eq!(summary_report.completed_items, 2);
    }
}
