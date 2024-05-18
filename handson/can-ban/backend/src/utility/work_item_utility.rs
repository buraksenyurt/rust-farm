use crate::models::WorkItem;
use chrono::{DateTime, Duration, Local};
use shared::*;

pub fn calculate_planned_finish_time(work_item: &WorkItem) -> Option<DateTime<Local>> {
    if let (Some(duration), Some(duration_type)) = (work_item.duration, work_item.duration_type) {
        let duration = match duration_type {
            DurationType::Hour => Duration::hours(duration as i64),
            DurationType::Day => Duration::days(duration as i64),
            DurationType::Week => Duration::weeks(duration as i64),
            DurationType::Month => Duration::days((duration * 30) as i64),
            DurationType::Unknown => return None,
        };
        return Some(work_item.crate_date + duration);
    }
    None
}
