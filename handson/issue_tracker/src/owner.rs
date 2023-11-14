use crate::json::Serializer;

#[derive(Debug)]
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
        json.push_str("{");
        json.push_str(&format!("\"name\": \"{}\",", self.name));
        json.push_str(&format!("\"last_name\": \"{}\"", self.last_name));
        json.push_str("}");
        json
    }
}
