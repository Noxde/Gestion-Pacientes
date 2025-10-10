use rusqlite::{Connection, Result};
use std::{fs, path::PathBuf};

pub fn init_db(app_data_dir: PathBuf) -> Result<Connection, String> {
    let db_path = app_data_dir.join("db.sqlite");

    if !db_path.exists() {
        fs::create_dir_all(app_data_dir)
            .map_err(|e| format!("Failed to create database directory: {}", e.to_string()))?;
    };

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Database connection failed: {}", e.to_string()))?;

    // Embed te schema at compile time
    let schema = include_str!("../../schema.sql");

    conn.execute_batch(&schema)
        .map_err(|e| format!("Failed to create tables: {}", e.to_string()))?;

    println!("Db connection successful: {}", db_path.to_str().unwrap());

    Ok(conn)
}
