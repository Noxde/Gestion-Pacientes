use rusqlite::{Connection, Result};
use std::{path::PathBuf, fs};

pub fn init_db(app_data_dir: PathBuf) -> Result<Connection, String> {
    let db_path = app_data_dir.join("db.sqlite");

    if !db_path.exists() {
        fs::create_dir_all(app_data_dir).map_err(|e| format!("Failed to create database directory: {}", e.to_string()))?;
    };

    let conn = Connection::open(&db_path).map_err(|e| format!("Database connection failed: {}", e.to_string()))?;

    conn.execute_batch(
        "
        BEGIN;
        CREATE TABLE IF NOT EXISTS patients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        COMMIT;
        "
    ).map_err(|e| format!("Failed to create tables: {}", e.to_string()))?;

    println!("Db connection successful: {}", db_path.to_str().unwrap());

    Ok(conn)
}
