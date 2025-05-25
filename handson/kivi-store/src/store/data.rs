use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
#[allow(dead_code)]
pub struct DataStore {
    context: Arc<Mutex<HashMap<String, String>>>,
}

#[allow(dead_code)]
impl DataStore {
    pub fn new() -> Self {
        DataStore{
            context: Arc::new(Mutex::new(HashMap::new()))
        }
    }
    
    pub fn set(&self, key: &str, value: &str) {
        self.context.lock().unwrap().insert(key.to_string(), value.to_string());
    }

    pub fn remove(&self, key: &str) {
        self.context.lock().unwrap().remove(key);
    }
    
    pub fn get(&self, key: &str) -> Option<String> {
        self.context.lock().unwrap().get(key).cloned()
    }
    
    pub fn keys(&self) -> Vec<String> {
        self.context.lock().unwrap().keys().cloned().collect()
    }
}