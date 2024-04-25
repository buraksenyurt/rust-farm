#[cfg(test)]
mod test {
    use crate::constants::MAX_SCREEN_WIDTH;
    use crate::entity::*;
    use crate::instrument::*;
    use crate::lane_manager::{Column, LaneManager};
    use crate::question_manager::QuestionManager;

    #[test]
    fn create_rect_test() {
        let position = Position::new(50, 10);
        let rect = Rectangle::new(
            position,
            Size::new(64, 64),
            "#5dade2".to_string(),
            Velocity::new(1, 1),
        );
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(-5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string(), velocity);
        rect.move_to();
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string(), velocity);
        rect.move_to();
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, -5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string(), velocity);
        rect.move_to();
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, 5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string(), velocity);
        rect.move_to();
        assert_eq!(rect.get_y(), 15);
    }

    #[test]
    fn get_lane_test() {
        let lane_manager = LaneManager::new();
        let expected = lane_manager.get_lane(Column::Zero);
        assert!(expected.is_some());
        assert_eq!(expected.unwrap().column, Column::Zero);
        assert_eq!(expected.unwrap().start, 0);
        assert_eq!(
            expected.unwrap().end,
            100 - BlockSize::Venti.to_size().width
        );
    }

    #[test]
    fn get_first_lane_range_test() {
        let lane_manager = LaneManager::new();
        let expected = lane_manager.get_lane_range(Column::Zero);
        assert_eq!(expected.start, 0);
        assert_eq!(expected.end, 100 - BlockSize::Venti.to_size().width as i32);
    }

    #[test]
    fn get_last_lane_range_test() {
        let lane_manager = LaneManager::new();
        let expected = lane_manager.get_lane_range(Column::Four);
        assert_eq!(expected.start, 400);
        assert_eq!(
            expected.end,
            (MAX_SCREEN_WIDTH - BlockSize::Venti.to_size().width) as i32
        );
    }

    #[test]
    fn get_first_question_test() {
        let question_manager = QuestionManager::init();
        let actual = question_manager.get_question(1);
        assert!(actual.is_some());
        assert_eq!(
            actual.unwrap().get_text(),
            "3.14 olarak da bilinen Matematik enstrümanıdır?".to_string()
        );
    }

    #[test]
    fn get_first_question_answers_test() {
        let question_manager = QuestionManager::init();
        let actual = question_manager.get_question(1);
        let answers = actual.unwrap().get_answers();
        assert_eq!(answers.len(), 5);
        assert_eq!(answers.iter().find(|a| a.is_correct()).iter().len(), 1);
    }
}
