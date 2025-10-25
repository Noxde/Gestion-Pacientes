use crate::custom_types::structs::{Visit, Doc};
use crate::db::common::save_docs;
use rusqlite::{Connection, Result};
use validator::Validate;
use std::path::{Path, PathBuf};

/// Save a new visit in the database (including docs)
pub fn save(
    mut new_visit: Visit,
    app_data_dir: &PathBuf,
    conn: &mut Connection,
) -> Result<Visit, String> {

    new_visit
        .validate()
        .map_err(|e| format!("Visit data validation error: {}", e))?;

    // Start transaction to allow rollback if docs fail
    let tx = conn.transaction()
        .map_err(|e| format!("DB Transaction error: {}", e))?;

    // Build query dynamically depending on datetime
    // Insert visit and obtain its ID
    let visit_id: i32 = if let Some(datetime) = new_visit.datetime {
        tx.query_row(
            "INSERT INTO visits (patient_id, title, reason, diagnosis, treatment, notes, datetime)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING id",
            (
                &new_visit.patient_id,
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
        tx.query_row(
            "INSERT INTO visits (patient_id, title, reason, diagnosis, treatment, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING id",
            (
                &new_visit.patient_id,
                &new_visit.title,
                &new_visit.reason,
                &new_visit.diagnosis,
                &new_visit.treatment,
                &new_visit.notes,
            ),
            |row| row.get(0),
        )
    }.map_err(|e| {
        if let rusqlite::Error::SqliteFailure(err, _) = &e {
            if err.code == rusqlite::ErrorCode::ConstraintViolation {
                return "Invalid patient_id: referenced patient does not exist".to_string();
            }
        }
        format!("Database error while saving visit: {}", e)
    })?;

    new_visit.id = visit_id;

    // Extract original file paths from new_visit.docs
    let file_paths: Vec<String> = new_visit.docs.iter().map(|d| d.path.clone()).collect();

    // Save docs
    let saved_docs: Vec<Doc> = save_docs(app_data_dir, visit_id, &file_paths, &tx)?;

    // Update visit docs to final stored docs
    new_visit.docs = saved_docs;

    // Commit only if everything succeeded
    tx.commit().map_err(|e| format!("Transaction commit failed: {}", e))?;

    Ok(new_visit)
}

pub fn get_all(patient_id: i32, data_dir: &Path, conn: &Connection) -> Result<Vec<Visit>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM visits WHERE patient_id = ?1")
        .map_err(|e| format!("Failed to prepare visits query: {}", e))?;

    let mut visits = stmt
        .query_map([patient_id], |row| {
            Ok(Visit {
                id: row.get("id")?,
                patient_id: row.get("patient_id")?,
                title: row.get("title")?,
                reason: row.get("reason")?,
                diagnosis: row.get("diagnosis")?,
                treatment: row.get("treatment")?,
                notes: row.get("notes")?,
                docs: vec![], // filled later
                datetime: row.get("datetime")?,
            })
        })
        .map_err(|e| format!("Failed to fetch visits: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse visits: {}", e))?;

    // Prepare once, reuse for all visits
    let mut stmt_docs = conn
        .prepare("SELECT id, name FROM docs WHERE visit_id = ?1")
        .map_err(|e| format!("Failed to prepare docs query: {}", e))?;

    for visit in &mut visits {
        let docs = stmt_docs
            .query_map([visit.id], |row| {
                let doc_id: i32 = row.get("id")?;
                let name: String = row.get("name")?;

                // Construct full path (e.g. /home/user/.local/share/app/docs/4)
                let path = data_dir
                    .join("docs")
                    .join(doc_id.to_string())
                    .to_string_lossy()
                    .into_owned();

                Ok(Doc { path, name })
            })
            .map_err(|e| format!("Failed to fetch docs for visit {}: {}", visit.id, e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to parse docs for visit {}: {}", visit.id, e))?;

        visit.docs = docs;
    }

    Ok(visits)
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

