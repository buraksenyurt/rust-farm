use crate::json::{Deserializer, Field, Serializer};

#[derive(Debug, Clone)]
pub struct Owner {
    pub name: String,
    pub last_name: String,
}

impl Owner {
    pub fn new(name: String, last_name: String) -> Self {
        Self { name, last_name }
    }
}

impl Serializer for Owner {
    fn to_json(&self) -> String {
        let mut json = String::new();
        json.push_str("\"owner\":");
        json.push('{');
        json.push_str(&format!("\"name\": \"{}\",", self.name));
        json.push_str(&format!("\"last_name\": \"{}\"", self.last_name));
        json.push('}');
        json
    }
}

impl Deserializer for Owner {
    fn from(json_content: &str) -> Result<Self, String> {
        let name_input = Field::get("name", json_content)?;
        let name = name_input.as_str()[2..name_input.len() - 1].to_string();
        let last_name_input = Field::get("last_name", json_content)?;
        let last_name = last_name_input.as_str()[2..last_name_input.len() - 1].to_string();
        Ok(Owner::new(name, last_name))
    }
}
