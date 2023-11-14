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
        if let Some(result) = input
            .find(field_name)
            .and_then(|i| input[i..input.len()].find(",").map(|j| (i, j)))
        {
            //println!("{}..{}", result.0, result.1);
            let pair = input[result.0 - 1..result.0 + result.1].chars();
            let pairs = pair.as_str().split(":").last();
            //println!("{} -> {}", field_name, pairs.unwrap_or_default());
            return Ok(pairs.unwrap_or_default().to_string());
        } else if let Some(result) = input
            .find(field_name)
            .and_then(|i| input[i..input.len()].find("}").map(|j| (i, j)))
        {
            let pair = input[result.0 - 1..result.0 + result.1].chars();
            let pairs = pair.as_str().split(":").last();
            return Ok(pairs.unwrap_or_default().to_string());
        };
        Err(format!("{} için Parse hatası", field_name))
    }
}
