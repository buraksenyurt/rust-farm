use crate::formatter::{Deserializer, Field, Serializer};
use crate::owner::Owner;
use crate::uuid::Uuid;
use std::io::{Error, ErrorKind, Write};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Issue {
    pub id: Uuid,
    pub title: String,
    pub state: IssueState,
    pub is_resolved: bool,
    pub owner: Owner,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IssueState {
    Warning,
    Critical,
    Error,
}

impl Issue {
    pub fn new(title: String, owner: Owner, state: IssueState, is_resolved: bool) -> Self {
        Self {
            id: Uuid::new(),
            title,
            state,
            owner,
            is_resolved,
        }
    }
}

impl Serializer for Issue {
    fn to_json(&self) -> String {
        let mut json = String::new();
        json.push('{');
        json.push_str(&format!("\"id\": \"{}\",", self.id));
        json.push_str(&format!("\"title\": \"{}\",", self.title));
        json.push_str(&format!("\"state\": \"{:?}\",", self.state));
        json.push_str(&format!("\"is_resolved\": {:?},", self.is_resolved));
        json.push_str(&self.owner.to_json());
        json.push('}');
        json
    }

    fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
        let mut bytes = Vec::new();

        bytes.write_all(&self.id.value.as_bytes())?;
        bytes.write_all(self.title.as_bytes())?;
        bytes.push(0_u8);
        match self.state {
            IssueState::Warning => bytes.write_all(&0_u8.to_ne_bytes())?,
            IssueState::Critical => bytes.write_all(&1_u8.to_ne_bytes())?,
            IssueState::Error => bytes.write_all(&2_u8.to_ne_bytes())?,
        }
        bytes.write_all(&[self.is_resolved as u8])?;
        if let Ok(owner_bytes) = &self.owner.to_bytes() {
            bytes.write_all(owner_bytes)?;
        }

        Ok(bytes)
    }
}
impl Deserializer for Issue {
    fn from(json_content: &str) -> Result<Issue, String>
    where
        Self: Sized,
    {
        let title_input = Field::get("title", json_content)?;
        let title = title_input.as_str()[2..title_input.len() - 1].to_string();
        let state_input = Field::get("state", json_content)?;
        let state = match state_input.as_str()[2..state_input.len() - 1].as_ref() {
            "Critical" => IssueState::Critical,
            "Error" => IssueState::Error,
            "Warning" => IssueState::Warning,
            _ => return Err("Geçersiz 'state' değeri".to_string()),
        };
        let mut resolved = false;
        if let Ok(is_resolved_input) = Field::get("is_resolved", json_content) {
            if let Ok(is_resolved) = bool::from_str(is_resolved_input.as_str().trim()) {
                resolved = is_resolved;
            }
        };

        let owner = <Owner as Deserializer>::from(json_content)?;
        Ok(Issue::new(title, owner, state, resolved))
    }

    fn from_bytes(content: &[u8]) -> std::io::Result<Self> {
        let content_length = content.len();
        let id_value = String::from_utf8_lossy(&content[0..32]).into_owned();
        let id = Uuid { value: id_value };
        println!("ID {}", id);
        /*
            title bilgisinin bittiği yeri bulmak için içeriğin 4ncü byte'ından itibaren
            tüm içeriğin uzunluğu kadar hareket edip o anki karakterin C-Style string'lerin bitiş
            noktasını ifade eden null byte ('\0') olup olmadığına bakılır.
            Böylece title bilgisinin bittiği yerin konumu bulunur. Benzer strateji,
            owner'ın name ve last_name içeriklerinin bulunmasında da kullanılır.
        */
        let title_end = content[32..]
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(content_length);
        let title = String::from_utf8_lossy(&content[32..title_end + 4]).into_owned();
        println!("Title {}", title);
        // devam eden byte içeriğindeki bilgi 0,1,2 olma durumuna göre IssueState enum sabitidir
        let state = match content[title_end + 5] {
            0 => IssueState::Warning,
            1 => IssueState::Critical,
            2 => IssueState::Error,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Geçersiz state. State value {:?}", content[title_end + 1]),
                ))
            }
        };
        println!("State {:?}", state);
        // yine takip eden bitin 0 veya 1 olma hali is_resolved için false, true olma halidir
        let is_resolved = content[title_end + 6] != 0;
        /*
            Bu kısımdan itibaren Owner bilgisi başlar.
            Başlığın bittiği konumdan sonra gelen state ve is_resolved konumlarına göre
            3 birim ileri gidilerek name ve last_name kısımları bulunur.
        */
        println!("Is Resolved {}", is_resolved);
        let owner_slice = &content[(title_end + 7)..];
        let owner = Owner::from_bytes(owner_slice).unwrap();

        Ok(Issue {
            id,
            title,
            state,
            is_resolved,
            owner,
        })
    }
}
