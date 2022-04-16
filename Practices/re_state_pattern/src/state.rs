// Oyun nesnesinin o anki state bilgisini alma davranışını tanımladığımız trait
pub trait State {
    fn get_state(&self) -> String;
}
