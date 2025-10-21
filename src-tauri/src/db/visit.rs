use crate::custom_types::structs::Visit;
use rusqlite::{Connection, Result};
use validator::Validate;

/// Save a new visit in the database
pub fn save(mut new_visit: Visit, patient_id: i32, conn: &Connection) -> Result<Visit, String> {
    new_visit
        .validate()
        .map_err(|e| format!("Visit data validation error: {}", e))?;

    // Build query dynamically depending on datetime
    let result = if let Some(datetime) = new_visit.datetime {
        conn.query_one(
            "INSERT INTO visits (patient_id, title, reason, diagnosis, treatment, notes, datetime)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id",
            (
                &patient_id,
                &new_visit.title,
                &new_visit.reason,
                &new_visit.diagnosis,
                &new_visit.treatment,
                &new_visit.notes,
                &datetime,
            ),
            |row| row.get(0),
        )
    } else {
        // Let SQLite use DEFAULT CURRENT_TIMESTAMP
        conn.query_one(
            "INSERT INTO visits (patient_id, title, reason, diagnosis, treatment, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING id",
            (
                &patient_id,
                &new_visit.title,
                &new_visit.reason,
                &new_visit.diagnosis,
                &new_visit.treatment,
                &new_visit.notes,
            ),
            |row| row.get(0),
        )
    };

    match result {
        Ok(id) => {
            new_visit.id = id;
            Ok(new_visit)
        }
        Err(rusqlite::Error::SqliteFailure(e, _)) if e.extended_code == 787 => {
            Err("Invalid patient_id: referenced patient does not exist".to_string())
        }
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            Err(format!("A database error occurred while saving the visit: {}", e))
        }
        Err(e) => Err(format!("Failed to save the new visit: {}", e)),
    }
}

/// Get all visits of a patient
pub fn get_all(patient_id: i32, conn: &Connection) -> Result<Vec<Visit>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM visits WHERE patient_id = ?1")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let visits_iter = stmt
        .query_map([&patient_id], |row| {
            Ok(Visit {
                id: row.get("id")?,
                patient_id: row.get("patient_id")?,
                title: row.get("title")?,
                reason: row.get("reason")?,
                diagnosis: row.get("diagnosis")?,
                treatment: row.get("treatment")?,
                notes: row.get("notes")?,
                files: vec![], // Files stored elsewhere (not part of visits table)
                datetime: row.get("datetime")?,
            })
        })
        .map_err(|e| format!("Failed to get visits: {}", e))?;

    let visits: Result<Vec<Visit>, _> = visits_iter.collect();
    visits.map_err(|e| format!("Failed to collect visits: {}", e))
}

/// Update a visit (does not modify id or patient_id)
pub fn update(visit: Visit, conn: &Connection) -> Result<Visit, String> {
    visit
        .validate()
        .map_err(|e| format!("Visit data validation error: {}", e))?;

    // Build query dynamically depending on datetime
    let result = if let Some(datetime) = visit.datetime {
        conn.execute(
            "UPDATE visits
             SET title = ?1, reason = ?2, diagnosis = ?3, treatment = ?4, notes = ?5, datetime = ?6
             WHERE id = ?7",
            (
                &visit.title,
                &visit.reason,
                &visit.diagnosis,
                &visit.treatment,
                &visit.notes,
                &datetime,
                &visit.id,
            ),
        )
    } else {
        // Skip updating datetime if None
        conn.execute(
            "UPDATE visits
             SET title = ?1, reason = ?2, diagnosis = ?3, treatment = ?4, notes = ?5
             WHERE id = ?6",
            (
                &visit.title,
                &visit.reason,
                &visit.diagnosis,
                &visit.treatment,
                &visit.notes,
                &visit.id,
            ),
        )
    };

    match result {
        Ok(_) => Ok(visit),
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            Err(format!("A database error occurred while updating the visit: {}", e))
        }
        Err(e) => Err(format!("Failed to update the visit: {}", e)),
    }
}

