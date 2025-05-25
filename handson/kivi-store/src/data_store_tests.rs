#[cfg(test)]
mod tests{
    use crate::store::DataStore;

    #[test]
    fn test_set_and_get(){
        let data_store = DataStore::new();
        data_store.set("Resilience","on");
        let expected = data_store.get("Resilience").unwrap();
        assert_eq!(expected, "on");
    }
}