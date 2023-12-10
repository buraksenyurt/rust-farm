use time::{Date, Duration, PrimitiveDateTime as DateTime, Time};

pub fn after(start: DateTime) -> DateTime {
    //start.add(Duration::seconds(1_000_000_000))
    start + Duration::seconds(1_000_000_000)
}

pub fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn date() {
        let start_date = dt(2011, 4, 25, 0, 0, 0);
        assert_eq!(after(start_date), dt(2043, 1, 1, 1, 46, 40));
    }

    #[test]
    fn another_date() {
        let start_date = dt(1977, 6, 13, 0, 0, 0);
        assert_eq!(after(start_date), dt(2009, 2, 19, 1, 46, 40));
    }

    #[test]
    fn third_date() {
        let start_date = dt(1959, 7, 19, 0, 0, 0);
        assert_eq!(after(start_date), dt(1991, 3, 27, 1, 46, 40));
    }

    #[test]
    fn datetime() {
        let start_date = dt(2015, 1, 24, 22, 0, 0);
        assert_eq!(after(start_date), dt(2046, 10, 2, 23, 46, 40));
    }

    #[test]
    fn another_datetime() {
        let start_date = dt(2015, 1, 24, 23, 59, 59);
        assert_eq!(after(start_date), dt(2046, 10, 3, 1, 46, 39));
    }
}
