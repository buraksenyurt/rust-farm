#[derive(Debug)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub state: IssueState,
    pub is_resolved: bool,
    pub owner: String,
}

#[derive(Debug)]
pub enum IssueState {
    Warning,
    Critical,
    Error,
}

impl Issue {
    pub fn new(id: i32, title: String, owner: String, state: IssueState) -> Self {
        Self {
            id,
            title,
            state,
            owner,
            is_resolved: false,
        }
    }

    pub fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{");
        json.push_str(&format!("\"id\": {},", self.id));
        json.push_str(&format!("\"title\": \"{}\",", self.title));
        json.push_str(&format!("\"state\": \"{:?}\",", self.state));
        json.push_str(&format!("\"is_resolved\": {:?},", self.is_resolved));
        json.push_str(&format!("\"owner\": \"{}\"", self.owner));
        json.push_str("}");
        json
    }
}
