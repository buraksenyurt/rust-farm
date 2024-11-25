pub trait Reportable {
    fn generate(template: &str, data: &Self) -> String
    where
        Self: serde::Serialize;
}
