//! Parse Nginx log lines formatted in combined log format
//!
//! Expected log line format:
//! 192.168.1.10 - - [25/Oct/2026:13:55:36 +0300] "GET /api/products HTTP/1.1" 200 1534 "-" "Mozilla/5.0" 0.123
#[derive(Debug, Clone, PartialEq)]
pub struct LogEntry {
    pub ip: String,
    pub timestamp: String,
    pub hour: u8,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub bytes: u64,
    pub user_agent: String,
    pub request_time: Option<f64>,
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingField(&'static str),
    InvalidNumber(&'static str),
}

/// Parse a single log line
pub fn parse_line(line: &str) -> Result<LogEntry, ParseError> {
    let line = line.trim();
    let (ip, rest) = line.split_once(' ').ok_or(ParseError::MissingField("ip"))?;
    let rest = rest
        .split_once('[')
        .ok_or(ParseError::MissingField("timestamp"))?
        .1;

    let (timestamp, rest) = rest
        .split_once(']')
        .ok_or(ParseError::MissingField("timestamp"))?;

    let hour = timestamp
        .split(':')
        .nth(1)
        .and_then(|h| h.parse::<u8>().ok())
        .ok_or(ParseError::InvalidNumber("hour"))?;

    let (request, rest) = take_quoted(rest).ok_or(ParseError::MissingField("request"))?;
    let mut parts = request.split_whitespace();
    let method = parts.next().ok_or(ParseError::MissingField("method"))?;
    let path = parts.next().ok_or(ParseError::MissingField("path"))?;
    let mut nums = rest.split_whitespace();
    let status = nums
        .next()
        .ok_or(ParseError::MissingField("status"))?
        .parse::<u16>()
        .map_err(|_| ParseError::InvalidNumber("status"))?;

    let bytes = match nums.next().ok_or(ParseError::MissingField("bytes"))? {
        "-" => 0,
        b => b
            .parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber("bytes"))?,
    };

    let (_referer, rest) = take_quoted(rest).ok_or(ParseError::MissingField("referer"))?;
    let (user_agent, rest) = take_quoted(rest).ok_or(ParseError::MissingField("user_agent"))?;
    let request_time = rest.trim().parse::<f64>().ok();

    Ok(LogEntry {
        ip: ip.to_string(),
        timestamp: timestamp.to_string(),
        hour,
        method: method.to_string(),
        path: path.to_string(),
        status,
        bytes,
        user_agent: user_agent.to_string(),
        request_time,
    })
}

/// Returns the first "..." block and the rest of the string.
fn take_quoted(s: &str) -> Option<(&str, &str)> {
    let start = s.find('"')? + 1;
    let len = s[start..].find('"')?;
    Some((&s[start..start + len], &s[start + len + 1..]))
}

/// Parse result: successful entries and number of failed lines.
#[derive(Debug, Default, Clone)]
pub struct ParseResult {
    pub entries: Vec<LogEntry>,
    pub failed: usize,
}

/// Parses a multi-line text; skips empty lines.
pub fn parse_all(text: &str) -> ParseResult {
    let mut result = ParseResult::default();

    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        match parse_line(line) {
            Ok(entry) => result.entries.push(entry),

            Err(_) => result.failed += 1,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"192.168.1.10 - - [25/Oct/2026:13:55:36 +0300] "GET /api/products?id=3 HTTP/1.1" 200 1534 "-" "Mozilla/5.0 (X11; Linux x86_64)" 0.123"#;

    #[test]
    fn parses_full_line() {
        let e = parse_line(SAMPLE).unwrap();
        assert_eq!(e.ip, "192.168.1.10");
        assert_eq!(e.hour, 13);
        assert_eq!(e.method, "GET");
        assert_eq!(e.path, "/api/products?id=3");
        assert_eq!(e.status, 200);
        assert_eq!(e.bytes, 1534);
        assert_eq!(e.user_agent, "Mozilla/5.0 (X11; Linux x86_64)");
        assert_eq!(e.request_time, Some(0.123));
    }

    #[test]
    fn request_time_is_optional() {
        let line = SAMPLE.trim_end_matches(" 0.123");
        assert_eq!(parse_line(line).unwrap().request_time, None);
    }

    #[test]
    fn dash_bytes_means_zero() {
        let line = SAMPLE.replace(" 1534 ", " - ");
        assert_eq!(parse_line(&line).unwrap().bytes, 0);
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse_line("bu bir log satırı değil").is_err());
    }

    #[test]
    fn parse_all_counts_failures() {
        let text = format!("{SAMPLE}\n\nbozuk satır\n{SAMPLE}\n");
        let r = parse_all(&text);
        assert_eq!(r.entries.len(), 2);
        assert_eq!(r.failed, 1);
    }
}
