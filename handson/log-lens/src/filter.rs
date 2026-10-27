//! Filtering criteria and matching logic.
use crate::parser::LogEntry;
use leptos::prelude::*;

/// Raw fitering criteria that can be applied to log entries.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Criteria {
    /// 2, 3, 4, 5 => 2xx, 3xx...
    pub status_class: Option<u16>,
    pub method: Option<String>,
    pub path_contains: String,
    pub min_ms: Option<f64>,
}

impl Criteria {
    pub fn matches(&self, e: &LogEntry) -> bool {
        if let Some(c) = self.status_class {
            if e.status / 100 != c {
                return false;
            }
        }

        if let Some(m) = &self.method {
            if &e.method != m {
                return false;
            }
        }

        if !self.path_contains.is_empty() && !e.path.contains(&self.path_contains) {
            return false;
        }

        if let Some(min) = self.min_ms {
            match e.request_time {
                Some(t) if t * 1000.0 >= min => {}
                _ => return false,
            }
        }
        true
    }
}

/// Reactive filter state tied to the form fields in the UI.
/// Provides a convenient way to manage and reset filter state in the UI.
#[derive(Clone, Copy)]
pub struct Filters {
    pub status: RwSignal<String>,
    pub method: RwSignal<String>,
    pub path: RwSignal<String>,
    pub min_ms: RwSignal<String>,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            status: RwSignal::new(String::new()),
            method: RwSignal::new(String::new()),
            path: RwSignal::new(String::new()),
            min_ms: RwSignal::new(String::new()),
        }
    }

    /// Converts the current reactive filter state into a plain `Criteria` instance.
    pub fn criteria(&self) -> Criteria {
        Criteria {
            status_class: self.status.get().parse().ok(),
            method: Some(self.method.get()).filter(|m| !m.is_empty()),
            path_contains: self.path.get().trim().to_string(),
            min_ms: self.min_ms.get().parse().ok(),
        }
    }

    pub fn reset(&self) {
        self.status.set(String::new());
        self.method.set(String::new());
        self.path.set(String::new());
        self.min_ms.set(String::new());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_line;

    fn e(method: &str, path: &str, status: u16, rt: f64) -> LogEntry {
        parse_line(&format!(
            r#"1.1.1.1 - - [25/Oct/2026:10:00:00 +0300] "{method} {path} HTTP/1.1" {status} 1 "-" "ua" {rt}"#
        ))
        .unwrap()
    }

    #[test]
    fn empty_criteria_matches_all() {
        assert!(Criteria::default().matches(&e("GET", "/", 200, 0.1)));
    }

    #[test]
    fn combined_criteria() {
        let c = Criteria {
            status_class: Some(5),
            method: Some("POST".into()),
            path_contains: "checkout".into(),
            min_ms: Some(500.0),
        };

        assert!(c.matches(&e("POST", "/api/checkout", 502, 0.9)));
        assert!(!c.matches(&e("POST", "/api/checkout", 200, 0.9)));
        assert!(!c.matches(&e("GET", "/api/checkout", 502, 0.9)));
        assert!(!c.matches(&e("POST", "/api/cart", 502, 0.9)));
        assert!(!c.matches(&e("POST", "/api/checkout", 502, 0.1)));
    }
}
