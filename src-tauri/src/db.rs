use crate::custom_types::structs::Patient;
use rusqlite::{Connection, Result};
use std::{fs, path::PathBuf};
use validator::Validate;

pub fn init_db(app_data_dir: PathBuf) -> Result<Connection, String> {
    let db_path = app_data_dir.join("db.sqlite");

    if !db_path.exists() {
        fs::create_dir_all(app_data_dir)
            .map_err(|e| format!("Failed to create database directory: {}", e.to_string()))?;
    };

    let conn = Connection::open(&db_path)
        .map_err(|e| format!("Database connection failed: {}", e.to_string()))?;

    let schema = std::fs::read_to_string("schema.sql")
        .map_err(|e| format!("Failed to read database file: {}", e.to_string()))?;

    conn.execute_batch(&schema)
        .map_err(|e| format!("Failed to create tables: {}", e.to_string()))?;

    println!("Db connection successful: {}", db_path.to_str().unwrap());

    Ok(conn)
}

pub fn save_patient(mut new_patient: Patient, conn: &Connection) -> Result<Patient, String> {
    new_patient
        .validate()
        .map_err(|e| format!("Patient data validation error: {}", e))?;

    match conn
        .query_one(
            "INSERT INTO patients (name, surname, national_id, phone, medicare, medicare_number, sex, gender, description) 
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) RETURNING id",
            (
                &new_patient.name,
                &new_patient.surname,
                &new_patient.national_id,
                &new_patient.phone,
                &new_patient.medicare,
                &new_patient.medicare_number,
                &new_patient.sex,
                &new_patient.gender,
                &new_patient.description,
            ),
            |row| row.get(0),
        ) {
        Ok(id) => {
            new_patient.id = id;
            Ok(new_patient)
        }
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            if e.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE {
                Err("A patient with this National ID already exists.".to_string())
            } else {
                Err(format!("A database error occurred while saving the patient: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to save the new patient: {}", e)),
    }
}

pub fn get_patients(conn: &Connection) -> Result<Vec<Patient>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM patients")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let patients_iter = stmt
        .query_map([], |row| {
            Ok(Patient {
                id: row.get("id")?,
                name: row.get("name")?,
                surname: row.get("surname")?,
                national_id: row.get("national_id")?,
                phone: row.get("phone")?,
                medicare: row.get("medicare")?,
                medicare_number: row.get("medicare_number")?,
                sex: row.get("sex")?,
                gender: row.get("gender")?,
                description: row.get("description")?,
            })
        })
        .map_err(|e| format!("Failed to get patients: {}", e))?;

    let patients: Result<Vec<Patient>, _> = patients_iter.collect();
    patients.map_err(|e| format!("Failed to collect patients: {}", e))
}
