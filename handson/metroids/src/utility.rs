use std::collections::HashMap;
pub fn key_down(key_manager: &mut HashMap<String, bool>, key_name: String) {
    if !key_manager.contains_key(&key_name) {
        key_manager.entry(key_name).or_insert(true);
    } else {
        if let Some(x) = key_manager.get_mut(&key_name) {
            *x = true;
        }
    }
}

pub fn key_up(key_manager: &mut HashMap<String, bool>, key_name: String) {
    if !key_manager.contains_key(&key_name) {
        key_manager.entry(key_name).or_insert(false);
    } else {
        if let Some(x) = key_manager.get_mut(&key_name) {
            *x = false;
        }
    }
}

pub fn is_key_pressed(key_manager: &HashMap<String, bool>, value: &str) -> bool {
    key_manager.contains_key(&value.to_string())
        && key_manager.get(&value.to_string()) == Some(&true)
}
