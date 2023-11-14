use crate::issue::{Issue, IssueState};
use crate::json::Serializer;
use crate::owner::Owner;

pub fn get_dummy_issues() -> Vec<Issue> {
    vec![
        Issue::new(
            1,
            "txtPassword kontrolünde güvenlik açığı var.".to_string(),
            Owner::new("Bill".to_string(), "Geyts".to_string()),
            IssueState::Critical,
        ),
        Issue::new(
            2,
            "GetProductList metodunda hatalı Dependecy kullanımı söz konusu.".to_string(),
            Owner::new("Nadonna".to_string(), "De Lafuante".to_string()),
            IssueState::Error,
        ),
        Issue::new(
            3,
            "Konfigurasyon dosyasındaki db adları güvenli alana taşınmalı".to_string(),
            Owner::new("Bizim".to_string(), "Con Do".to_string()),
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
    json_array.push(']');
    json_array
}
