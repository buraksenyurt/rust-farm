use bytes::Bytes;
use std::collections::HashMap;

#[derive(Default)]
pub struct Db {
    store: HashMap<String, Bytes>,
}

impl Db {
    pub fn write<'a>(&mut self, key: String, value: String) -> Result<&str, &'a str> {
        let result = &self.store.insert(key, Bytes::from(value));
        match result {
            Some(_) => Ok("exists"),
            None => Ok("inserted"),
        }
    }

    pub fn read<'a>(&mut self, key: String) -> Result<&Bytes, &'a str> {
        let query_result = self.store.get(key.as_str());
        if let Some(value) = query_result {
            Ok(value)
        } else {
            Err("value bulunamadı")
        }
    }
}
