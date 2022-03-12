use serde_json::{Map, Value};
use std::fs::File;
use std::io::Read;

/// Dosyadan, JSON formatındaki Work Item bilgilerini okuyup geriye döner
pub fn read_file(file_name: &str) -> Result<Map<String, Value>, String> {
    let mut file = File::open(file_name.to_string()).expect("Dosya açma hatası");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Okuma hatası");
    let json: Value = serde_json::from_str(&content).expect("JSON Convert hatası");
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    Ok(state)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn non_exist_file_should_be_panic() {
        let _ = read_file("unknown.json");
    }

    #[test]
    fn json_file_exist_and_readable() {
        let file_path = format!("{}/states.json", env!("CARGO_MANIFEST_DIR"));
        let result = read_file(file_path.as_str());
        match result {
            Ok(m) => {
                let v = m.get("oda temizleme");
                assert!(v.unwrap().is_object())
            }
            _ => {}
        }
    }
}
