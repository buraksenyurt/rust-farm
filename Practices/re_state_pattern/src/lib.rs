/*
   Bu örnekte bir Oyun nesnesinin Menu, Playing ve EndGame konumlarında olma halini ele aldığımız
   bir state pattern örneği söz konusu.

   Oyun nesnesi Menu konumundan, Playing konumuna geçebilir ama EndGame konumuna geçemez.
   EndGame konumundan tekrar Playing'e (Replay seçeneği) veya Menu durumuna geçebilir.
*/

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct Game {
    game_id: u32,
    status: Box<dyn GameStateAction>,
}

trait State {
    fn get_state(&self) -> String;
}
trait GameStateAction: State {
    fn playing(&mut self) -> bool;
    fn end_game(&mut self) -> bool;
    fn go_to_menu(&mut self) -> bool;
    fn play_again(&mut self) -> bool;
}
