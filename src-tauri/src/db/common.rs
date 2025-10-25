use rusqlite::{Connection, Result, Transaction};
use std::{fs,
    path::{Path, PathBuf}};
use crate::custom_types::structs::Doc;

pub fn init_db(app_data_dir: &PathBuf) -> Result<Connection, String> {
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

///Copy the files into app_data_dir/docs/<id>
///Save their info in the DB as `docs` tuples (does not commit tx)
pub fn save_docs(
    app_data_dir: &PathBuf,
    visit_id: i32,
    paths: &Vec<String>,
    tx: &Transaction,
) -> Result<Vec<Doc>, String> {
    let mut saved_docs = Vec::new();

    // Ensure base docs directory exists
    let docs_dir = app_data_dir.join("docs");
    fs::create_dir_all(&docs_dir)
        .map_err(|e| format!("Failed to create docs directory: {}", e))?;

    for original_path in paths {
        let original_path = Path::new(original_path);

        let file_name = original_path.file_name()
            .ok_or_else(|| format!("{}: Invalid file path.", original_path.display()))?
            .to_string_lossy()
            .to_string();

        // 1. Insert db record and get id
        let doc_id: i32 = tx.query_row(
            "INSERT INTO docs (visit_id, name) VALUES (?1, ?2) RETURNING id",
            (&visit_id, &file_name),
            |row| row.get(0),
        ).map_err(|e| format!("{}: Database insert error: {}", file_name, e))?;

        // 2. Destination path: .../docs/<doc_id>
        let dest_path = docs_dir.join(doc_id.to_string());

        // 3. Copy file
        fs::copy(&original_path, &dest_path)
            .map_err(|e| format!("{}: File copy error: {}", file_name, e))?;

        // 4. Build Doc struct
        saved_docs.push(Doc {
            path: dest_path.to_string_lossy().into_owned(),
            name: file_name,
        });
    }

    Ok(saved_docs)
}
