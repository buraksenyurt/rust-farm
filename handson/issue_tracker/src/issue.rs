use crate::json::{Deserializer, Field, Serializer};
use crate::owner::Owner;
use std::str::FromStr;

#[derive(Debug)]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub state: IssueState,
    pub is_resolved: bool,
    pub owner: Owner,
}

#[derive(Debug, PartialEq)]
pub enum IssueState {
    Warning,
    Critical,
    Error,
}

impl Issue {
    pub fn new(id: i32, title: String, owner: Owner, state: IssueState) -> Self {
        Self {
            id,
            title,
            state,
            owner,
            is_resolved: false,
        }
    }
}

impl Serializer for Issue {
    fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("{");
        json.push_str(&format!("\"id\": {},", self.id));
        json.push_str(&format!("\"title\": \"{}\",", self.title));
        json.push_str(&format!("\"state\": \"{:?}\",", self.state));
        json.push_str(&format!("\"is_resolved\": {:?},", self.is_resolved));
        json.push_str(&format!("{}", self.owner.to_json()));
        json.push_str("}");
        json
    }
}
impl Deserializer for Issue {
    fn from(json_content: &str) -> Result<Issue, String> {
        println!("Content -> {}", json_content);
        let id = Field::get("id", &json_content)?;
        let title = Field::get("title", &json_content)?;
        let state_input = Field::get("state", &json_content)?;
        let state = match state_input.as_str() {
            "Critical" => IssueState::Critical,
            "Error" => IssueState::Error,
            "Warning" => IssueState::Warning,
            _ => return Err("Geçersiz 'state' değeri".to_string()),
        };
        let owner = <Owner as Deserializer>::from(json_content)?;
        Ok(Issue::new(
            i32::from_str(id.as_str()).unwrap(),
            title,
            owner,
            state,
        ))
    }
}
