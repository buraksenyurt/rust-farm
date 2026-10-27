//! Raw computation of log statistics.
use crate::parser::LogEntry;
use std::collections::HashSet;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Summary {
    pub total: usize,
    pub client_errors: usize, // 4xx
    pub server_errors: usize, // 5xx
    pub unique_ips: usize,
    pub total_bytes: u64,
    pub avg_ms: Option<f64>,
    pub p95_ms: Option<f64>,
}

impl Summary {
    pub fn error_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            (self.client_errors + self.server_errors) as f64 * 100.0 / self.total as f64
        }
    }
}

pub fn summarize<'a>(entries: impl Iterator<Item = &'a LogEntry>) -> Summary {
    let mut s = Summary::default();
    let mut ips = HashSet::new();
    let mut times = Vec::new();
    for e in entries {
        s.total += 1;
        s.total_bytes += e.bytes;
        match e.status {
            400..=499 => s.client_errors += 1,
            500..=599 => s.server_errors += 1,
            _ => {}
        }
        ips.insert(e.ip.as_str());
        if let Some(t) = e.request_time {
            times.push(t * 1000.0);
        }
    }

    s.unique_ips = ips.len();
    if !times.is_empty() {
        s.avg_ms = Some(times.iter().sum::<f64>() / times.len() as f64);
        times.sort_by(|a, b| a.total_cmp(b));
        let idx = ((times.len() as f64) * 0.95).ceil() as usize - 1;
        s.p95_ms = Some(times[idx.min(times.len() - 1)]);
    }
    s
}

/// 1536000 -> "1.5 MB"
pub fn human_bytes(b: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut v = b as f64;
    let mut i = 0;
    while v >= 1024.0 && i < UNITS.len() - 1 {
        v /= 1024.0;
        i += 1;
    }
    if i == 0 {
        format!("{b} B")
    } else {
        format!("{v:.1} {}", UNITS[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_line;

    fn entry(ip: &str, status: u16, rt: f64) -> LogEntry {
        let line = format!(
            r#"{ip} - - [25/Oct/2026:10:00:00 +0300] "GET / HTTP/1.1" {status} 100 "-" "ua" {rt}"#
        );
        parse_line(&line).unwrap()
    }

    #[test]
    fn summary_counts() {
        let data = vec![
            entry("1.1.1.1", 200, 0.1),
            entry("1.1.1.1", 404, 0.2),
            entry("2.2.2.2", 500, 0.3),
            entry("3.3.3.3", 200, 0.4),
        ];
        let s = summarize(data.iter());
        assert_eq!(s.total, 4);
        assert_eq!(s.client_errors, 1);
        assert_eq!(s.server_errors, 1);
        assert_eq!(s.unique_ips, 3);
        assert_eq!(s.total_bytes, 400);
        assert_eq!(s.error_rate(), 50.0);
        assert!((s.avg_ms.unwrap() - 250.0).abs() < 1e-9);
        assert!((s.p95_ms.unwrap() - 400.0).abs() < 1e-9);
    }

    #[test]
    fn empty_summary() {
        let s = summarize(std::iter::empty());
        assert_eq!(s.total, 0);
        assert_eq!(s.p95_ms, None);
    }

    #[test]
    fn bytes_formatting() {
        assert_eq!(human_bytes(512), "512 B");
        assert_eq!(human_bytes(1536), "1.5 KB");
        assert_eq!(human_bytes(5 * 1024 * 1024), "5.0 MB");
    }
}
