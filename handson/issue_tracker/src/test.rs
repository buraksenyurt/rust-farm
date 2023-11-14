#[cfg(test)]
pub mod tests {
    use crate::data::get_dummy_issues;
    use crate::issue::{Issue, IssueState};
    use crate::json::{Deserializer, Serializer};
    use crate::owner::Owner;

    #[test]
    pub fn dummy_issues_data_is_not_empty_test() {
        let issues = get_dummy_issues();
        assert!(!issues.is_empty());
    }

    #[test]
    pub fn issue_to_json_works_test() {
        let issue = Issue::new(
            99,
            "Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli".to_string(),
            Owner::new("Administrator".to_string(), "System".to_string()),
            IssueState::Warning,
        );
        let expected = issue.to_json();
        let actual = "{\"id\": 99,\"title\": \"Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli\",\"state\": \"Warning\",\"is_resolved\": false,\"owner\":{\"name\": \"Administrator\",\"last_name\": \"System\"}}";
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn from_json_to_issue_works_test() {
        let json_content = r#"{"id": 10001,"title": "Windows Server'lar için Upgrade çalışması","state": "Warning","owner":{"name":"martin","last_name":"mystery"}}"#;
        let issue = <Issue as Deserializer>::from(json_content);
        let issue = issue.unwrap();
        assert_eq!(issue.id, 10001);
        assert_eq!(issue.title, "Windows Server'lar için Upgrade çalışması");
        assert_eq!(issue.state, IssueState::Warning);
        assert_eq!(issue.owner.name, "martin");
        assert_eq!(issue.owner.last_name, "mystery");
    }
}
