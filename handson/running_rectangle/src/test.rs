#[cfg(test)]
mod test {
    use crate::constants::MAX_SCREEN_WIDTH;
    use crate::lane_manager::{Column, LaneManager};
    use crate::{BlockSize, Position, Rectangle, Size, Velocity};

    #[test]
    fn create_rect_test() {
        let position = Position::new(50, 10);
        let rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        assert_eq!(rect.get_x(), 50);
        assert_eq!(rect.get_y(), 10);
        assert_eq!(rect.get_width(), 64);
        assert_eq!(rect.get_height(), 64);
    }

    #[test]
    fn move_left_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(-5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 45);
    }

    #[test]
    fn move_right_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(5, 0);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_x(), 55);
    }

    #[test]
    fn move_up_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, -5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
        assert_eq!(rect.get_y(), 5);
    }

    #[test]
    fn move_down_rect_test() {
        let position = Position::new(50, 10);
        let velocity = Velocity::new(0, 5);
        let mut rect = Rectangle::new(position, Size::new(64, 64), "#5dade2".to_string());
        rect.move_to(velocity);
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
}
