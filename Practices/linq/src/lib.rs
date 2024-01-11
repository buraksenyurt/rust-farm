mod domain;
mod query;

#[cfg(test)]
mod tests {
    use crate::domain::{Expertise, Genre, Order};
    use crate::query::GameQueryEngine;

    #[test]
    fn test_get_games_by_rating_with_grated_than_and_equal() {
        let curious = GameQueryEngine::init();
        let result = curious.get_by_rate_gte(1.).len();
        assert!(result > 0);
    }

    #[test]
    fn test_get_only_game_names_ordered_by_asc() {
        let mut curious = GameQueryEngine::init();
        let names = curious.get_game_names(Order::Ascending);
        assert_eq!(names[0], "Battlefield Glory");
    }

    #[test]
    fn test_get_only_game_names_ordered_by_desc() {
        let mut curious = GameQueryEngine::init();
        let names = curious.get_game_names(Order::Descending);
        assert_eq!(names[0], "Treasure Island");
    }

    #[test]
    fn test_get_average_rate() {
        let curious = GameQueryEngine::init();
        let avg_rate = curious.get_average_rate();
        assert_eq!(avg_rate, 4.3454547);
    }

    #[test]
    fn test_grouped_by_genre() {
        let curious = GameQueryEngine::init();
        let grouped = curious.grouped_by_genre();
        assert!(grouped.get(&Genre::Action).is_some());
        assert_eq!(grouped.keys().len(), 6);
    }

    #[test]
    fn test_get_higher_rated_game() {
        let curious = GameQueryEngine::init();
        let most_popular = curious.get_higher_rated_game();
        assert!(most_popular.is_some());
    }

    #[test]
    fn test_grouped_by_genre_with_game_count() {
        let curious = GameQueryEngine::init();
        let grouped = curious.grouped_by_genre();
        assert!(grouped.get(&Genre::Action).is_some());
        assert_eq!(grouped.get(&Genre::Action).unwrap().len(), 2);
    }

    #[test]
    fn test_get_games_by_specific_expertise() {
        let curious = GameQueryEngine::init();
        let games = curious.get_games_by_specific_expertise(Expertise::AI);
        assert_eq!(games.len(), 3);
    }

    #[test]
    fn test_get_oldest_games() {
        let mut curious = GameQueryEngine::init();
        let games = curious.get_oldest_games();
        assert!(games.is_some());
        if let (Some(result)) = games {
            assert_eq!(result.len(), 3);
        }
    }
}
