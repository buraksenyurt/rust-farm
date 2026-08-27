use chrono::prelude::*;
use chrono::LocalResult;
use rusqlite::{params, Connection, Result as SqlResult, Row};

use crate::models::{External, MediaType, Note, NoteInput};

pub fn init_db(conn: &Connection) -> SqlResult<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            body TEXT NOT NULL,
            publisher TEXT NOT NULL,
            author TEXT NOT NULL,
            media_type TEXT NOT NULL,
            year INTEGER NOT NULL,
            month TEXT NOT NULL,
            day INTEGER NOT NULL,
            is_archived INTEGER NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS externals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            note_id INTEGER NOT NULL REFERENCES notes(id) ON DELETE CASCADE,
            title TEXT NOT NULL,
            url TEXT NOT NULL
        );",
    )
}

fn base_note_from_row(row: &Row) -> SqlResult<Note> {
    let media_type_str: String = row.get(5)?;
    let is_archived: i64 = row.get(9)?;
    Ok(Note {
        id: row.get(0)?,
        title: row.get(1)?,
        body: row.get(2)?,
        publisher: row.get(3)?,
        author: row.get(4)?,
        media_type: MediaType::from_str(&media_type_str),
        year: row.get(6)?,
        month: row.get(7)?,
        day: row.get(8)?,
        externals: Vec::new(),
        is_archived: is_archived != 0,
    })
}

fn fetch_externals(conn: &Connection, note_id: i64) -> SqlResult<Vec<External>> {
    let mut stmt = conn.prepare("SELECT title, url FROM externals WHERE note_id = ?1 ORDER BY id")?;
    let externals = stmt
        .query_map(params![note_id], |row| {
            Ok(External {
                title: row.get(0)?,
                url: row.get(1)?,
            })
        })?
        .collect();
    externals
}

const NOTE_COLUMNS: &str =
    "id, title, body, publisher, author, media_type, year, month, day, is_archived";

pub fn get_random_note(conn: &Connection) -> SqlResult<Option<Note>> {
    let sql = format!(
        "SELECT {NOTE_COLUMNS} FROM notes WHERE is_archived = 0 ORDER BY RANDOM() LIMIT 1"
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map([], base_note_from_row)?;
    match rows.next() {
        Some(note) => {
            let mut note = note?;
            note.externals = fetch_externals(conn, note.id)?;
            Ok(Some(note))
        }
        None => Ok(None),
    }
}

pub fn list_notes(conn: &Connection) -> SqlResult<Vec<Note>> {
    let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE is_archived = 0");
    let mut stmt = conn.prepare(&sql)?;
    let mut notes = stmt
        .query_map([], base_note_from_row)?
        .collect::<SqlResult<Vec<_>>>()?;
    for note in notes.iter_mut() {
        note.externals = fetch_externals(conn, note.id)?;
    }
    Ok(notes)
}

/// Orijinal warp uygulamasındaki `get_date_from` mantığının aynısı: Türkçe ay adını sayıya
/// çevirir, geçersiz gün/ay/yıl kombinasyonlarında 1976-12-04'e düşer (fallback tarih).
fn sort_date(note: &Note) -> DateTime<Utc> {
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
        1..=31 => note.day as u32,
        _ => 1,
    };

    match Utc.with_ymd_and_hms(note.year as i32, month, day, 0, 0, 0) {
        LocalResult::Single(dt) => dt,
        _ => DateTime::parse_from_str("1976-12-04 00:00", "%Y-%m-%d %H:%M")
            .unwrap()
            .to_utc(),
    }
}

pub fn list_notes_sorted(conn: &Connection, column: &str, order: &str) -> SqlResult<Vec<Note>> {
    let mut notes = list_notes(conn)?;
    let desc = order == "desc";

    match column {
        "title" => notes.sort_by(|n1, n2| {
            if desc {
                n2.title.cmp(&n1.title)
            } else {
                n1.title.cmp(&n2.title)
            }
        }),
        "author" => notes.sort_by(|n1, n2| {
            if desc {
                n2.author.cmp(&n1.author)
            } else {
                n1.author.cmp(&n2.author)
            }
        }),
        "id" => notes.sort_by(|n1, n2| {
            if desc {
                n2.id.cmp(&n1.id)
            } else {
                n1.id.cmp(&n2.id)
            }
        }),
        "date" => notes.sort_by(|n1, n2| {
            if desc {
                sort_date(n2).cmp(&sort_date(n1))
            } else {
                sort_date(n1).cmp(&sort_date(n2))
            }
        }),
        _ => notes.sort_by(|n1, n2| n1.title.cmp(&n2.title)),
    }

    Ok(notes)
}

pub fn get_note(conn: &Connection, id: i64) -> SqlResult<Option<Note>> {
    let sql = format!("SELECT {NOTE_COLUMNS} FROM notes WHERE id = ?1");
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query_map(params![id], base_note_from_row)?;
    match rows.next() {
        Some(note) => {
            let mut note = note?;
            note.externals = fetch_externals(conn, note.id)?;
            Ok(Some(note))
        }
        None => Ok(None),
    }
}

pub fn add_note(conn: &Connection, input: NoteInput) -> SqlResult<Note> {
    conn.execute(
        "INSERT INTO notes (title, body, publisher, author, media_type, year, month, day, is_archived)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 0)",
        params![
            input.title,
            input.body,
            input.publisher,
            input.author,
            input.media_type.as_str(),
            input.year,
            input.month,
            input.day,
        ],
    )?;
    let id = conn.last_insert_rowid();
    for ext in &input.externals {
        conn.execute(
            "INSERT INTO externals (note_id, title, url) VALUES (?1, ?2, ?3)",
            params![id, ext.title, ext.url],
        )?;
    }
    get_note(conn, id).map(|note| note.expect("just inserted note must exist"))
}

pub fn archive_note(conn: &Connection, id: i64) -> SqlResult<()> {
    conn.execute("UPDATE notes SET is_archived = 1 WHERE id = ?1", params![id])?;
    Ok(())
}
