#[cfg(test)]
mod test {
    use crate::controller::Controller;
    use crate::todo::Todo;

    #[test]
    pub fn should_new_task_not_completed_test() {
        let trigo = Todo::new(1, "Trigonometri 101 dersini tekrar et".to_string());
        assert_eq!(trigo.completed, false);
    }

    #[test]
    pub fn should_controller_add_works_test() {
        let trigo = Todo::new(1, "Trigonometri 101 dersini tekrar et".to_string());
        let walk = Todo::new(2, "Gün içinde 100 adım at".to_string());
        let puzzle = Todo::new(3, "Bugün bir tane çengel bulmaca çöz".to_string());
        let mut controller = Controller::default();
        controller.add(trigo);
        controller.add(walk);
        controller.add(puzzle);
        assert_eq!(controller.list().iter().filter(|t| t.completed).count(), 0);
        assert_eq!(controller.list().len(), 3);
    }

    #[test]
    pub fn should_get_task_works_test() {
        let trigo = Todo::new(1, "Trigonometri 101 dersini tekrar et".to_string());
        let puzzle = Todo::new(3, "Bugün bir tane çengel bulmaca çöz".to_string());
        let mut controller = Controller::default();
        controller.add(trigo);
        controller.add(puzzle);
        assert!(controller.get(3).is_some());
        assert_eq!(controller.get(99), None);
    }

    #[test]
    pub fn should_complete_task_works_test() {
        let trigo = Todo::new(1, "Trigonometri 101 dersini tekrar et".to_string());
        let mut controller = Controller::default();
        controller.add(trigo);
        controller.complete(1);
        assert_eq!(controller.is_completed(1), true);
    }
}
