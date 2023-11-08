use crate::issue::{Issue, IssueState};

pub fn get_dummy_issues() -> Vec<Issue> {
    vec![
        Issue::new(
            1,
            "txtPassword kontrolünde güvenlik açığı var.".to_string(),
            "Bill Geyts".to_string(),
            IssueState::Critical,
        ),
        Issue::new(
            2,
            "GetProductList metodunda hatalı Dependecy kullanımı söz konusu.".to_string(),
            "Nadonna De Lafuante".to_string(),
            IssueState::Error,
        ),
        Issue::new(
            3,
            "Konfigurasyon dosyasındaki db adları güvenli alana taşınmalı".to_string(),
            "Bizim Con Do".to_string(),
            IssueState::Warning,
        ),
    ]
}

pub fn to_json_array(issues: &Vec<Issue>) -> String {
    let mut json_array = String::from("[\n");
    for (i, issue) in issues.iter().enumerate() {
        json_array.push_str(&issue.to_json());
        if i < issues.len() - 1 {
            json_array.push_str(",\n");
        }
    }
    json_array.push_str("]");
    json_array
}

#[cfg(test)]
pub mod tests {
    use crate::data::get_dummy_issues;
    use crate::issue::{Issue, IssueState};

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
            "Administrator".to_string(),
            IssueState::Warning,
        );
        let expected = issue.to_json();
        let actual = "{\"id\": 99,\"title\": \"Load Balancer'da bilinmeyen kesintiler söz konusu. İncelenmeli\",\"state\": \"Warning\",\"is_resolved\": false,\"owner\": \"Administrator\"}";
        assert_eq!(expected, actual);
    }
}
