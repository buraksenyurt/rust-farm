mod concrete_strategy;
mod context;
mod entity;
mod strategy;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concrete_strategy::*;
    use crate::context::SortingMaster;
    use crate::entity::Player;

    #[test]
    fn quick_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Quick);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn bubble_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Bubble);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn insertion_sort_test() {
        let mut numbers = vec![3, 6, 1, 2, 9, 12, 23, 8, 90, 45, 24, 7, 8];
        let sorter = SortingMaster::new(Insertion);
        sorter.sort(&mut numbers);
        let expected = vec![1, 2, 3, 6, 7, 8, 8, 9, 12, 23, 24, 45, 90];
        assert_eq!(numbers, expected);
    }

    #[test]
    fn insertion_sort_on_player_list_test() {
        let mut players = vec![
            Player::new(1, "John Doe", 9.5),
            Player::new(2, "Darth Veydar", 5.5),
            Player::new(3, "Kristin", 8.5),
            Player::new(4, "Sandra Makarena", 7.5),
            Player::new(5, "Super Maryo", 2.5),
        ];
        let sorter = SortingMaster::new(Insertion);
        sorter.sort(&mut players);
        let expected = vec![
            Player::new(5, "Super Maryo", 2.5),
            Player::new(2, "Darth Veydar", 5.5),
            Player::new(4, "Sandra Makarena", 7.5),
            Player::new(3, "Kristin", 8.5),
            Player::new(1, "John Doe", 9.5),
        ];
        assert_eq!(players, expected);
    }
}
