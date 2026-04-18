use rusqlite::{params, Connection, OptionalExtension};
use directories::ProjectDirs;
use anyhow::{Result, Context};

pub fn init_db() -> Result<Connection> {
    let proj_dirs = ProjectDirs::from("com", "pasteplus", "pasteplus")
        .context("Could not determine config directory")?;
    
    let db_path = proj_dirs.data_dir();
    std::fs::create_dir_all(db_path)?;
    
    let conn = Connection::open(db_path.join("history.db"))?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS history (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            pinned INTEGER DEFAULT 0
        )",
        [],
    )?;

    Ok(conn)
}

pub fn insert_item(conn: &Connection, content: &str) -> Result<()> {
    let last_content: Option<String> = conn.query_row(
        "SELECT content FROM history ORDER BY timestamp DESC LIMIT 1",
        [],
        |row| row.get(0),
    ).optional()?;

    if let Some(last) = last_content {
        if last == content {
            return Ok(());
        }
    }

    conn.execute(
        "INSERT INTO history (content) VALUES (?)",
        params![content],
    )?;

    conn.execute(
        "DELETE FROM history WHERE id NOT IN (
            SELECT id FROM history WHERE pinned = 1 
            UNION 
            SELECT id FROM (
                SELECT id FROM history 
                ORDER BY timestamp DESC LIMIT 50
            )
        )",
        [],
    )?;

    Ok(())
}

pub fn delete_item(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM history WHERE id = ?", params![id])?;
    Ok(())
}

pub fn toggle_pin(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE history SET pinned = 1 - pinned WHERE id = ?",
        params![id],
    )?;
    Ok(())
}