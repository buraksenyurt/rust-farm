#[cfg(test)]
mod test {
    use crate::request::Request;
    use std::str::FromStr;

    #[test]
    fn should_request_from_string_works_test() {
        let message = "GET salary/categories/2 HTTP";
        let request = Request::from_str(message).unwrap();
        assert_eq!(request.method, Some("GET".to_string()));
        assert_eq!(request.path, Some("salary/categories/2".to_string()));
        assert_eq!(request.protocol, Some("HTTP".to_string()));
    }
}
