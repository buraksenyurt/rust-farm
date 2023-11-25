#[cfg(test)]
pub mod tests {
    use crate::data::IssueStore;
    use crate::formatter::{Deserializer, Serializer};
    use crate::issue::{Issue, IssueState};
    use crate::owner::Owner;
    use crate::request::{Request, RequestMethod};
    use std::str::FromStr;

    #[test]
    pub fn dummy_issues_data_is_not_empty_test() {
        let mut issues = IssueStore::default();
        issues.seed();
        assert!(!issues.data.is_empty());
    }
    #[test]
    pub fn issue_to_json_works_test() {
        let issue = Issue::new(
            99,
            "Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli".to_string(),
            Owner::new("Administrator".to_string(), "System".to_string()),
            IssueState::Warning,
            false,
        );
        let expected = issue.to_json();
        let actual = "{\"id\": 99,\"title\": \"Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli\",\"state\": \"Warning\",\"is_resolved\": false,\"owner\":{\"name\": \"Administrator\",\"last_name\": \"System\"}}";
        assert_eq!(expected, actual);
    }
    #[test]
    pub fn from_json_to_issue_works_test() {
        let json_content = r#"{"id": 10001,"title": "Windows Server'lar için Upgrade çalışması","state": "Warning","owner": {"name": "martin","last_name": "mystery"}}"#;
        let issue = <Issue as Deserializer>::from(json_content);
        let issue = issue.unwrap();
        assert_eq!(issue.id, 10001);
        assert_eq!(issue.title, "Windows Server'lar için Upgrade çalışması");
        assert_eq!(issue.state, IssueState::Warning);
        assert_eq!(issue.owner.name, "martin");
        assert_eq!(issue.owner.last_name, "mystery");
    }
    #[test]
    pub fn convert_from_valid_get_request_line_works_test() {
        let line = "GET /issues/1 HTTP/1.1";
        let request = Request::from_str(line);
        assert!(request.is_ok());
        let actual = request.unwrap();
        let expected = Request::new(
            RequestMethod::Get,
            "HTTP/1.1".to_string(),
            "/issues/1".to_string(),
        );
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn convert_from_valid_get_all_request_line_works_test() {
        let line = "GET /issues HTTP/1.1";
        let request = Request::from_str(line);
        assert!(request.is_ok());
        let actual = request.unwrap();
        let expected = Request::new(
            RequestMethod::Get,
            "HTTP/1.1".to_string(),
            "/issues".to_string(),
        );
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn convert_from_valid_delete_request_line_works_test() {
        let line = "DELETE /issues/1 HTTP/1.1";
        let request = Request::from_str(line);
        assert!(request.is_ok());
        let actual = request.unwrap();
        let expected = Request::new(
            RequestMethod::Delete,
            "HTTP/1.1".to_string(),
            "/issues/1".to_string(),
        );
        assert_eq!(actual, expected);
    }
    #[test]
    pub fn convert_from_valid_post_request_line_works_test() {
        let line = "POST /issues HTTP/1.1";
        let request = Request::from_str(line);
        assert!(request.is_ok());
        let actual = request.unwrap();
        let expected = Request::new(
            RequestMethod::Post,
            "HTTP/1.1".to_string(),
            "/issues".to_string(),
        );
        assert_eq!(actual, expected);
    }

    #[test]
    pub fn issue_serialize_to_bytes_test() {
        let issue = Issue::new(
            99,
            "Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli.".to_string(),
            Owner::new("Administrator".to_string(), "System".to_string()),
            IssueState::Warning,
            false,
        );
        let issue_bytes = issue.to_bytes();
        assert!(issue_bytes.is_ok());
        assert_eq!(issue_bytes.unwrap().len(), 93);
    }

    #[test]
    pub fn owner_serialize_to_bytes_test() {
        let owner = Owner::new("Administrator".to_string(), "System".to_string());
        let issue_bytes = owner.to_bytes();
        assert!(issue_bytes.is_ok());
        assert_eq!(issue_bytes.unwrap().len(), 21);
    }

    #[test]
    pub fn issue_deserialize_from_bytes_test() {
        let issue = Issue::new(
            99,
            "Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli.".to_string(),
            Owner::new("Administrator".to_string(), "System".to_string()),
            IssueState::Warning,
            false,
        );
        let binding = issue.to_bytes().unwrap();
        let source_bytes = binding.as_slice();
        let issue = Issue::from_bytes(source_bytes).unwrap();
        assert_eq!(issue.id, 99);
        assert_eq!(issue.is_resolved, false);
        assert_eq!(
            issue.title,
            "Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli.".to_string()
        );
        assert_eq!(issue.owner.name, "Administrator".to_string());
        assert_eq!(issue.owner.last_name, "System".to_string());
        assert_eq!(issue.state, IssueState::Warning);
    }
}
