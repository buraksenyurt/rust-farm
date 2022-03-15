pub struct Storage {}

impl Storage {
    pub fn get() -> String {
        format!("{}/states.json", env!("CARGO_MANIFEST_DIR"))
    }
}
