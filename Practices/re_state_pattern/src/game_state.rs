use crate::state::State;

// Oyun nesnesinin dahil olabileceği durumlar birer davranış olarak bu trait içinde tanımlanır.
// Başlangıç konumu için init,
// oyun oynanıyorken playing,
// oyuncu yandığında end_game,
// oyuncu yandıktan sonra tekrar oynamak istediğinde play_again
// oyuncu yandıktan sonra menüye dönmek isterse menu
pub trait GameState: State {
    fn init(&mut self) -> bool;
    fn playing(&mut self) -> bool;
    fn end_game(&mut self) -> bool;
    fn play_again(&mut self) -> bool;
    fn menu(&mut self) -> bool;
}
