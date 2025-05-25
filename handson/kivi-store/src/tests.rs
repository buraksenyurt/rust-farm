#[cfg(test)]
mod tests {
    use crate::command::Command;
    use crate::store::DataStore;

    #[tokio::test]
    async fn test_set_and_get() {
        let data_store = DataStore::new();
        data_store.set("Resilience", "on").await;
        let expected = data_store.get("Resilience").await.unwrap();
        assert_eq!(expected, "on");
    }

    #[test]
    fn test_command_parse() {
        let cmd = Command::parse("SET Connection dataSource=localhost;database=MongoDb");
        match cmd {
            Command::Set { key, value } => {
                assert_eq!(key, "Connection");
                assert_eq!(value, "dataSource=localhost;database=MongoDb");
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }
    #[test]
    fn test_invalid_command() {
        let cmd = Command::parse("INPUT Connection dataSource=localhost;database=MongoDb");
        match cmd {
            Command::Invalid(input) => {
                assert_eq!(input, "INPUT")
            }
            _ => panic!("Expected to parse SET command!"),
        }
    }
}
