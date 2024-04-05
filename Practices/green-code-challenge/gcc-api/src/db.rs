use crate::model::{Challenge, DurationType};
use rusqlite::{params, Connection, Error, Result, Row};

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

pub fn delete_challenge(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM challenges WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn select_challenges(conn: &Connection) -> Result<Vec<Challenge>> {
    let mut stmt = conn.prepare("SELECT id, serial_code, title, duration, duration_type, purpose, details, stakeholders, expected_outputs, benefits FROM challenges")?;
    let challenge_iter = stmt.query_map([], |row| get_from(row)?)?;

    let mut challenges = Vec::new();
    for challenge in challenge_iter {
        challenges.push(challenge?);
    }
    Ok(challenges)
}

fn get_from(row: &Row) -> Result<Result<Challenge, Error>, Error> {
    Ok(Ok(Challenge {
        id: row.get(0)?,
        serial_code: row.get(1)?,
        title: row.get(2)?,
        duration: row.get(3)?,
        duration_type: match row.get::<_, String>(4)?.as_str() {
            "Day" => DurationType::Day,
            "Week" => DurationType::Week,
            "Month" => DurationType::Month,
            _ => unreachable!(),
        },
        purpose: row.get(5)?,
        details: row.get(6)?,
        stakeholders: row.get(7)?,
        expected_outputs: row.get(8)?,
        benefits: row.get(9)?,
    }))
}

pub fn select_challenge_by_id(conn: &Connection, id: i32) -> Result<Option<Challenge>> {
    let mut stmt = conn.prepare("SELECT id, serial_code, title, duration, duration_type, purpose, details, stakeholders, expected_outputs, benefits FROM challenges WHERE id = ?1")?;

    let mut challenge_iter = stmt.query_map(params![id], |row| get_from(row)?)?;

    if let Some(result) = challenge_iter.next() {
        result.map(Some)
    } else {
        Ok(None)
    }
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
