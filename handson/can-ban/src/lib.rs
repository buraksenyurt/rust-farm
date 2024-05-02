use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WorkItem {
    id: u32,
    title: String,
    duration: Option<u32>,
    duration_type: Option<DurationType>,
    size: Option<Size>,
    status: Status,
}

#[wasm_bindgen]
pub enum DurationType {
    Hour,
    Day,
    Week,
    Month,
}

#[wasm_bindgen]
pub enum Size {
    Small,
    Medium,
    Large,
    Epic,
}

#[derive(Debug, PartialEq, Clone)]
#[wasm_bindgen]
pub enum Status {
    Todo,
    Inprogress,
    Completed,
}

impl WorkItem {
    pub fn new(id: u32, title: String) -> Self {
        Self {
            id,
            title,
            duration: None,
            duration_type: None,
            size: None,
            status: Status::Todo,
        }
    }
    pub fn get_current_status(&self) -> Status {
        self.status.clone()
    }
    pub fn change_status(&mut self, new_status: Status) {
        self.status = new_status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_work_item_status_must_at_todo_state_test() {
        let work_item = WorkItem::new(1, "Design the system".to_string());
        assert_eq!(work_item.get_current_status(), Status::Todo);
    }

    #[test]
    fn change_status_from_test() {
        let mut work_item = WorkItem::new(1, "Design the system".to_string());
        work_item.change_status(Status::Inprogress);
        assert_eq!(work_item.get_current_status(), Status::Inprogress);
    }
}
