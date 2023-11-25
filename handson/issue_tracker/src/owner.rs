use crate::formatter::{Deserializer, Field, Serializer};
use std::io::Write;

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

    fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.write_all(self.name.as_bytes())?;
        bytes.push(0_u8);
        bytes.write_all(self.last_name.as_bytes())?;
        bytes.push(0_u8);

        Ok(bytes)
    }
}

impl Deserializer for Owner {
    fn from(json_content: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        let name_input = Field::get("name", json_content)?;
        let name = name_input.as_str()[2..name_input.len() - 1].to_string();
        let last_name_input = Field::get("last_name", json_content)?;
        let last_name = last_name_input.as_str()[2..last_name_input.len() - 1].to_string();
        Ok(Owner::new(name, last_name))
    }

    fn from_bytes(content: &[u8]) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        let name_end = content[..].iter().position(|&x| x == 0).unwrap_or(0);
        let name = String::from_utf8_lossy(&content[..name_end]).into_owned();
        println!("Owner Name {} Name end {}", name, name_end);
        let last_name =
            String::from_utf8_lossy(&content[name_end + 1..content.len() - 1]).into_owned();
        println!("Last Name {}", last_name);
        Ok(Self { name, last_name })
    }
}
