use crate::model::{Challenge, DurationType};
use rusqlite::{params, Connection, Result};

pub fn create_challenge(conn: &Connection, challenge: Challenge) -> Result<()> {
    let duration_type = match challenge.duration_type {
        DurationType::Day => "Day",
        DurationType::Week => "Week",
        DurationType::Month => "Month",
    };

    conn.execute(
        "INSERT INTO challenges (serial_code, title, duration, duration_type, purpose, details, stakeholders, expected_outputs, benefits)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            challenge.serial_code,
            challenge.title,
            challenge.duration,
            duration_type,
            challenge.purpose,
            challenge.details,
            challenge.stakeholders,
            challenge.expected_outputs,
            challenge.benefits,
        ],
    )?;
    Ok(())
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS challenges (
            id INTEGER PRIMARY KEY,
            serial_code TEXT NOT NULL,
            title TEXT NOT NULL,
            duration INTEGER NOT NULL,
            duration_type TEXT NOT NULL,
            purpose TEXT NOT NULL,
            details TEXT NOT NULL,
            stakeholders TEXT NOT NULL,
            expected_outputs TEXT NOT NULL,
            benefits TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}
