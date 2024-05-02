use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WorkItemManager {}

#[wasm_bindgen]
impl WorkItemManager {
    pub fn new() -> Self {
        Self {}
    }
    pub fn create(&self, title: String, duration: i32, durationType: String, size: String) -> i32 {
        1
    }
}

pub struct WorkItem {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}

pub enum DurationType {
    Hour,
    Day,
    Week,
    Month,
}

pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
}

pub enum Status {
    Todo,
    Inprogress,
    Completed,
}
