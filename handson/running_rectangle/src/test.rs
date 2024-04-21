#[cfg(test)]
mod test {
    use super::*;
    use crate::{Position, Rectangle, Size, Velocity};

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
}
