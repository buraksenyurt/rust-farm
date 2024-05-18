#[cfg(test)]
pub mod utility_test {
    use crate::{calculate_planned_finish_time, WorkItem};
    use chrono::{Duration, Local};
    use shared::{DurationType, Status};

    #[test]
    fn test_calculate_planned_finish_time_with_hours() {
        let create_date = Local::now();
        let mut work_item = WorkItem {
            id: 0,
            title: "Test Work Item".to_string(),
            duration: Some(2),
            duration_type: Some(DurationType::Hour),
            size: None,
            status: Status::Todo,
            crate_date: create_date,
            finish_date: None,
            modified_date: None,
        };
        let calculated = calculate_planned_finish_time(&work_item);
        work_item.finish_date = calculated;
        assert_eq!(create_date + Duration::hours(2), calculated.unwrap());
    }

    #[test]
    fn test_calculate_planned_finish_time_with_days() {
        let create_date = Local::now();
        let mut work_item = WorkItem {
            id: 0,
            title: "Test Work Item".to_string(),
            duration: Some(3),
            duration_type: Some(DurationType::Day),
            size: None,
            status: Status::Todo,
            crate_date: create_date,
            finish_date: None,
            modified_date: None,
        };
        let calculated = calculate_planned_finish_time(&work_item);
        work_item.finish_date = calculated;
        assert_eq!(create_date + Duration::days(3), calculated.unwrap());
    }
}
