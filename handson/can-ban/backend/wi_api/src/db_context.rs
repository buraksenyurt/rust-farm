use crate::model::{
    CreateWorkItemResponse, DurationType, Size, Status, UpdateStatusRequest, WorkItem,
};
use rusqlite::{params, Connection, Result};

pub struct DbContext {
    pub conn: Connection,
}

impl DbContext {
    pub fn new() -> Result<Self> {
        let conn = Connection::open("can_ban.db")?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS work_items (
                    id INTEGER PRIMARY KEY,
                    title TEXT NOT NULL,
                    duration INTEGER NOT NULL,
                    duration_type INTEGER NOT NULL,
                    size INTEGER NOT NULL,
                    status INTEGER NOT NULL,
                    archived INTEGER DEFAULT 0,
                    create_date TEXT NOT NULL,
                    modified_date TEXT
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_work_item(&self, item: &WorkItem) -> Result<u32> {
        self.conn.execute(
            "INSERT INTO work_items (title, duration, duration_type, size, status, create_date)
                VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                item.title,
                item.duration,
                item.duration_type.unwrap() as u8,
                item.size.unwrap() as u8,
                item.status as u8,
                item.crate_date.to_rfc3339()
            ],
        )?;
        Ok(self.conn.last_insert_rowid() as u32)
    }

    pub fn update_work_item_status(&self, payload: &UpdateStatusRequest) -> Result<()> {
        self.conn.execute(
            "UPDATE work_items SET status = ?1 WHERE id= ?2",
            params![payload.new_status as u8, payload.id],
        )?;
        Ok(())
    }

    pub fn move_to_archive(&self, id: u32) -> Result<()> {
        self.conn.execute(
            "UPDATE work_items SET archived = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn get_item(&self, id: u32) -> Result<CreateWorkItemResponse, rusqlite::Error> {
        self.conn.query_row(
            "SELECT id, title, duration, duration_type, size, status FROM work_items WHERE id = ?1 AND archived <> 1",
            params![id],
            |row| {
                Ok(CreateWorkItemResponse {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    duration: row.get(2)?,
                    duration_type: row
                        .get::<_, Option<u8>>(3)?
                        .map(|dt| DurationType::try_from(dt).unwrap()),
                    size: row
                        .get::<_, Option<u8>>(4)?
                        .map(|sz| Size::try_from(sz).unwrap()),
                    status: Status::try_from(row.get::<_, u8>(5)?).unwrap(),
                })
            },
        )
    }

    pub fn get_all(&self) -> Result<Vec<CreateWorkItemResponse>, rusqlite::Error> {
        let mut query = self.conn.prepare(
            "SELECT id,title,duration,duration_type,size,status FROM work_items ORDER BY id",
        )?;
        let reader = query.query_map([], |row| {
            Ok(CreateWorkItemResponse {
                id: row.get(0)?,
                title: row.get(1)?,
                duration: row.get(2)?,
                duration_type: row
                    .get::<_, Option<u8>>(3)?
                    .map(|dt| DurationType::try_from(dt).unwrap()),
                size: row
                    .get::<_, Option<u8>>(4)?
                    .map(|sz| Size::try_from(sz).unwrap()),
                status: Status::try_from(row.get::<_, u8>(5)?).unwrap(),
            })
        })?;

        let mut results = Vec::new();
        for row in reader {
            results.push(row?);
        }
        Ok(results)
    }
}
