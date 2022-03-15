use super::base::Base;
use super::traits::create::Create;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use super::traits::get::Get;

/// Plana dahil edilmiş bir işi temsil eder
#[derive(Debug, PartialEq)]
pub struct Ready {
    pub header: Base,
}

impl Ready {
    pub fn new(input_title: &str) -> Self {
        Ready {
            header: Base::new(input_title, "Ready"),
        }
    }
}

impl Create for Ready {}
impl Edit for Ready {}
impl Delete for Ready {}
impl Get for Ready {}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::{json, Map, Value};

    #[test]
    fn should_new_ready_works() {
        let job = Ready::new("Odayı temizle");
        assert_eq!(job.header.status, "Ready");
    }

    #[test]
    fn should_get_trait_works() {
        let mut sample_data: Map<String, Value> = Map::new();
        let v = json!({ "value": 5,"state": "Ready" });
        sample_data.insert("Odayı Temizle".to_string(), v);
        let v = json!({ "value": 3,"state": "Ready" });
        sample_data.insert("Kitap Oku".to_string(), v);

        let job = Ready::new("Odayı Temizle");
        let actual = job.get("Odayı Temizle", &sample_data);
        let expected = json!({ "value": 5,"state": "Ready" });
        assert_eq!(actual, Some(expected));
    }
}
