#[cfg(test)]
mod test {
    use crate::controller::Controller;
    use crate::repository::{read_db, write_db};
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
        //assert_eq!(controller.list().iter().filter(|t| t.completed).count(), 0);
        assert!(controller.list().len() > 0);
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

    #[test]
    pub fn should_write_tasks_to_file_works_test() {
        let trigo = Todo::new(1, "Trigonometri 101 dersini tekrar et".to_string());
        let puzzle = Todo::new(3, "Bugün bir tane çengel bulmaca çöz".to_string());
        let mut controller = Controller::default();
        controller.add(trigo);
        controller.add(puzzle);

        let result = write_db(controller.list());
        assert_eq!(result, true);
    }

    #[test]
    pub fn should_read_file_works_test() {
        let result = read_db();
        assert!(result.len() > 0);
    }

    #[test]
    pub fn should_todo_format_works_test() {
        let trigo = Todo::new(99, "Trigonometri 101 dersini tekrar et".to_string());
        let formatted = trigo.format();
        assert_eq!(
            formatted,
            String::from("99|Trigonometri 101 dersini tekrar et|false")
        );
        let mut controller = Controller::default();
        controller.add(trigo);
        controller.complete(99);
        let formatted = controller.get(99);
        assert_eq!(
            formatted.unwrap().format(),
            String::from("99|Trigonometri 101 dersini tekrar et|true")
        );
    }
}
