use crate::issue::{Issue, IssueState};
use crate::owner::Owner;

#[derive(Default)]
pub struct IssueStore {
    pub data: Vec<Issue>,
}

impl IssueStore {
    pub fn seed(&mut self) {
        self.data = vec![
            Issue::new(
                "txtPassword kontrolünde güvenlik açığı var.".to_string(),
                Owner::new("Bill".to_string(), "Geyts".to_string()),
                IssueState::Critical,
                false,
            ),
            Issue::new(
                "GetProductList metodunda hatalı Dependecy kullanımı söz konusu.".to_string(),
                Owner::new("Nadonna".to_string(), "De Lafuante".to_string()),
                IssueState::Error,
                false,
            ),
            Issue::new(
                "Konfigurasyon dosyasındaki db adları güvenli alana taşınmalı".to_string(),
                Owner::new("Bizim".to_string(), "Con Do".to_string()),
                IssueState::Warning,
                false,
            ),
        ];
    }
}
