use crate::custom_types::structs::{Patient, MedicalHistory};
use crate::db::visit;
use rusqlite::{Connection, Result};
use validator::Validate;
use std::path::PathBuf;

pub fn save(mut new_patient: Patient, conn: &Connection) -> Result<Patient, String> {
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

pub fn get_all(conn: &Connection) -> Result<Vec<Patient>, String> {
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

pub fn update(patient: Patient, conn: &Connection) -> Result<Patient, String> {
    patient
        .validate()
        .map_err(|e| format!("Patient data validation error: {}", e))?;

    match conn.execute(
        "UPDATE patients 
        SET name = ?1, surname = ?2, national_id = ?3, phone = ?4, medicare = ?5, medicare_number = ?6, sex = ?7, gender = ?8, description = ?9 WHERE id = ?10",
        (
            &patient.name,
            &patient.surname,
            &patient.national_id,
            &patient.phone,
            &patient.medicare,
            &patient.medicare_number,
            &patient.sex,
            &patient.gender,
            &patient.description,
            patient.id,
        ),
    ) {
        Ok(_) => Ok(patient),
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            if e.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_UNIQUE {
                Err("A patient with this National ID already exists.".to_string())
            } else {
                Err(format!("A database error occurred while updating the patient: {}", e))
            }
        }
        Err(e) => Err(format!("Failed to update the patient: {}", e)),
    }
}

pub fn get(id: i32, conn: &Connection) -> Result<Patient, String> {
    conn.query_row(
        "SELECT * FROM patients WHERE id = ?1",
        [id],
        |row| {
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
        }
    )
    .map_err(|e| format!("Failed to load patient {}: {}", id, e))
}

pub fn get_medical_history(
    conn: &Connection,
    patient_id: i32,
    data_dir: &PathBuf,
) -> Result<MedicalHistory, String> {
    // 1. Fetch patient
    let patient = self::get(patient_id, conn)?;

    // 2. Fetch visits (this already loads docs and paths)
    let visits = visit::get_all(patient_id, data_dir, conn)?;

    // 3. Return complete medical history
    Ok(MedicalHistory { patient, visits })
}
