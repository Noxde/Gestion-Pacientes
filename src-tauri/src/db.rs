use rusqlite::{Connection, Result};
use std::{path::PathBuf, fs};
use crate::structs::Patient;

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

pub fn save_patient(mut new_patient: Patient, conn: &Connection) -> Result<Patient, String> {
    new_patient.id = conn.query_one("INSERT INTO patients (name) VALUES (?1) RETURNING id",
        (&new_patient.name,), |row| row.get(0),
        ).map_err(|e| format!("Failed to save the new patient: {}", e))?;

    Ok(new_patient)
}
pub fn get_patients(conn: &Connection) -> Result<Vec<Patient>, String> {
    let mut stmt = conn.prepare("SELECT id, name FROM patients").map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let patients_iter = stmt.query_map([], |row| {
        Ok(Patient {
            id: row.get(0)?,
            name: row.get(1)?,
        })}).map_err(|e| format!("Failed to map patients: {}", e))?;

    let patients: Result<Vec<Patient>, _> = patients_iter.collect();
    patients.map_err(|e| format!("Failed to collect patients: {}", e))
}
