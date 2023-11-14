pub trait Serializer {
    fn to_json(&self) -> String;
}
