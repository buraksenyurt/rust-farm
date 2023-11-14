pub trait Serializer {
    fn to_json(&self) -> String;
}

pub trait Deserializer {
    fn from(content: &str) -> Result<Self, String>
    where
        Self: Sized;
}

pub struct Field {}

impl Field {
    pub fn get(field_name: &str, input: &str) -> Result<String, String> {


        Err(format!("{} bulunamadı.", field_name))
    }
}
