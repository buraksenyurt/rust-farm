// Exercise #1 : Reversed String
pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    input.chars().rev().for_each(|c| result.push(c));
    result
}
// Exercise #2 : Gigasecond
use time::{Duration, PrimitiveDateTime as DateTime};

pub fn after(start: DateTime) -> DateTime {
    //start.add(Duration::seconds(1_000_000_000))
    start + Duration::seconds(1_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;
    use time::{Date, Month, PrimitiveDateTime, Time};

    #[test]
    fn should_cool_reversed_to_looc() {
        let value = "cool";
        let expected = "looc".to_string();
        let actual = reverse(value);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_unicode_reversed_correctly() {
        let value = "uüu";
        let expected = "uüu".to_string();
        let actual = reverse(value);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_a_gigasecond_after_correct_date() {
        let date = PrimitiveDateTime::new(
            Date::from_calendar_date(2015, Month::January, 24).unwrap(),
            Time::from_hms(22, 0, 0).unwrap(),
        );
        let start_date = DateTime::from(date);
        let future_date = PrimitiveDateTime::new(
            Date::from_calendar_date(2046, Month::October, 2).unwrap(),
            Time::from_hms(23, 46, 40).unwrap(),
        );
        let expected = DateTime::from(future_date);
        let actual = after(start_date);
        assert_eq!(expected, actual);
    }
}
