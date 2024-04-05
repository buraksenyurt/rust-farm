#[cfg(test)]
mod tests {
    use crate::db::{create_challenge, delete_challenge, init_db, select_challenges};
    use crate::model::{Challenge, DurationType};
    use rusqlite::Connection;

    fn setup_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        init_db(&conn).unwrap();
        conn
    }

    #[test]
    fn create_new_challenge_and_select_test() {
        let conn = setup_db();

        let new_challenge = create_dummy_challenge();

        create_challenge(&conn, new_challenge.clone()).unwrap();
        let challenges = select_challenges(&conn).unwrap();
        assert_eq!(challenges.len(), 1);
        assert_eq!(challenges[0].title, new_challenge.title);
    }
    #[test]
    fn create_and_delete_new_challenge_test() {
        let conn = setup_db();

        let new_challenge = create_dummy_challenge();

        create_challenge(&conn, new_challenge).unwrap();
        let challenges = select_challenges(&conn).unwrap();
        let id = challenges[0].id;

        delete_challenge(&conn, id).unwrap();
        let challenges_after_delete = select_challenges(&conn).unwrap();
        assert!(challenges_after_delete.is_empty());
    }

    fn create_dummy_challenge() -> Challenge {
        Challenge {
            id: 0,
            serial_code: "GCC-2024-01-70".to_string(), // Green Code Challenge, Year, Order, Priority(10..100)
            title: "Satış Raporlama Servisinin İyileştirilmesi".to_string(),
            duration: 10,
            duration_type: DurationType::Day,
            purpose: "Çok uzun süre çalışan raporlama servisinin daha enerji dostu...".to_string(),
            details: "Servise ait detaylar...".to_string(),
            stakeholders: "Yazılım,Raporlama,Uygulama Yönetimi,".to_string(),
            expected_outputs: "Ortalama 1 saatlik çalışma süresinin 10 dakika altına indirilmesi".to_string(),
            benefits: "Sene sonu süre kazanımına göre elde edilen enerji tasarrufunun yüzde 50si maliyetinde ağaçlandırma".to_string(),
        }
    }
}
