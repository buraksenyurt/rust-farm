use crate::entity::Note;
use chrono::prelude::*;
use chrono::LocalResult;
use log::error;
use std::env;

pub fn get_file_path(relative_path: &str) -> String {
    match env::var("DOCKER_ENV") {
        Ok(_) => format!("/usr/local/bin/{}", relative_path),
        Err(_) => relative_path.to_string(),
    }
}

pub fn get_date_from(note: &Note) -> DateTime<Utc> {
    let month = match note.month.as_str() {
        "Ocak" => 1,
        "Şubat" => 2,
        "Mart" => 3,
        "Nisan" => 4,
        "Mayıs" => 5,
        "Haziran" => 6,
        "Temmuz" => 7,
        "Ağustos" => 8,
        "Eylül" => 9,
        "Ekim" => 10,
        "Kasım" => 11,
        "Aralık" => 12,
        _ => 1,
    };
    let day = match note.day {
        1..=31 => note.day,
        _ => 1,
    };

    match Utc.with_ymd_and_hms(note.year as i32, month, day as u32, 00, 00, 00) {
        LocalResult::Single(dt) => dt,
        _ => {
            error!("Geçersiz tarih: {}-{}-{}", note.year, month, note.day);
            DateTime::parse_from_str("1976-12-04 00:00", "%Y-%m-%d %H:%M")
                .unwrap()
                .to_utc()
        }
    }
}
