use crate::custom_types::structs::Patient;
use rusqlite::{Connection, Result};
use validator::Validate;

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

// TODO: Return all patient data, including visits and their files
pub fn get_all_data(conn: &Connection, patient_id: i32) -> Result<Patient, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM patients WHERE patient_id = ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let patients_iter = stmt
        .query_map([patient_id], |row| {
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
    let p = patients.map_err(|e| format!("Failed to collect patients: {}", e));
    let p = p.unwrap();
    Ok(p[0].clone())
}
