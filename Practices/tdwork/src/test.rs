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
        let mut controller = Controller::default();
        controller.add("Trigonometri 101 dersini tekrar et".to_string());
        controller.add("Gün içinde 100 adım at".to_string());
        controller.add("Bugün bir tane çengel bulmaca çöz".to_string());
        assert!(controller.list().len() > 0);
    }

    #[test]
    pub fn should_get_task_works_test() {
        let mut controller = Controller::default();
        let task_id = controller.add("İlkokul öğretmeninin bir halini hatırını sor.".to_string());
        controller.add("Sarp'a matematik çalıştır.".to_string());
        assert!(controller.get(task_id).is_some());
        assert_eq!(controller.get(99), None);
    }

    #[test]
    pub fn should_complete_task_works_test() {
        let mut controller = Controller::default();
        let task_id = controller.add("Proje kodlarını refaktör et".to_string());
        controller.complete(task_id);
        assert_eq!(controller.is_completed(task_id), true);
    }

    #[test]
    pub fn should_write_tasks_to_file_works_test() {
        let mut controller = Controller::default();
        controller.add("Trigonometri 101 dersini tekrar et".to_string());
        controller.add("Bugün bir tane çengel bulmaca çöz".to_string());

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
        let trigo = Todo::new(
            99,
            "Oyun programlama giriş konularını gözden geçir".to_string(),
        );
        let formatted = trigo.format();
        assert_eq!(
            formatted,
            String::from("99|Oyun programlama giriş konularını gözden geçir|false")
        );
        let mut controller = Controller::default();
        let task_id = controller.add("Oyun programlama giriş konularını gözden geçir".to_string());
        controller.complete(task_id);
        let formatted = controller.get(task_id);
        assert_eq!(
            formatted.unwrap().format(),
            String::from(format!(
                "{}|Oyun programlama giriş konularını gözden geçir|true",
                task_id
            ))
        );
    }

    #[test]
    pub fn should_delete_todo_works_test() {
        let mut controller = Controller::default();
        controller.add("Bu hafta 3 saat İspaynolca çalış".to_string());
        let task_id = controller.add("10 Km bisiklet sür".to_string());
        controller.delete(task_id);
        assert_eq!(controller.get(task_id), None);
    }

    #[test]
    pub fn should_create_id_works_test() {
        let mut controller = Controller::default();
        controller.add("This week in Rust bülteninden bir konu çalış".to_string());
        let last_task_id = controller.add("Bir Codingame problemi çöz".to_string());
        let new_id = controller.create_id() - 1;
        assert_eq!(new_id, last_task_id);
    }
}
