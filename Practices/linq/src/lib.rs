use crate::domain::{Expertise, Game, Genre, Programmer};

mod domain;
mod query;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::query::GameQueryEngine;

    #[test]
    fn it_works() {
        let curious = GameQueryEngine::init();

        let result = curious.get_by_rate_gte(1.).len();
        assert!(result > 0);
    }
}
