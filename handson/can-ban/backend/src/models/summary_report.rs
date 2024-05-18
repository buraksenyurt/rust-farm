use serde::Serialize;

#[derive(Default, Serialize)]
pub struct SummaryReport {
    pub work_items: u32,
    pub todo_items: u32,
    pub in_progress_items: u32,
    pub completed_items: u32,
}
