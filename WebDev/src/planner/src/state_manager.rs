use log::info;
use serde_json::{json, Map, Value};
use std::fs;
use std::fs::File;
use std::io::Read;

/// Dosyadan, JSON formatındaki Work Item bilgilerini okuyup geriye döner
pub fn read_file(file_name: &str) -> Result<Map<String, Value>, String> {
    info!("{} dosyasından okuma işlemi.", file_name);
    let mut file = File::open(file_name.to_string()).expect("Dosya açma hatası");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Okuma hatası");
    let json: Value = serde_json::from_str(&content).expect("JSON Convert hatası");
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    Ok(state)
}

/// Work Item verisini JSON formatından dosyaya yazmak için kullanılır.
pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) -> Result<(), String> {
    info!("{} dosyasına yazma işlemi.", file_name);
    let json_data = json!(state);
    info!("Güncel içerik. {:#?}", json_data);
    fs::write(file_name.to_string(), json_data.to_string())
        .expect("Dosya yazma işlemi sırasında hata");
    Ok(())
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
    fn should_write_and_read_works() {
        let file_path = format!("{}/states.json", env!("CARGO_MANIFEST_DIR"));
        let mut sample_data: Map<String, Value> = Map::new();
        let v = json!({ "value": 3,"state": "Ready" });
        sample_data.insert("Rust Çalış".to_string(), v);
        let _ = write_to_file(file_path.as_str(), &mut sample_data);

        let result = read_file(file_path.as_str());
        match result {
            Ok(m) => {
                let v = m.get("Rust Çalış");
                assert!(v.unwrap().is_object())
            }
            _ => {}
        }
    }
}
