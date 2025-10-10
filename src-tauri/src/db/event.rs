use crate::custom_types::structs::Event;
use rusqlite::{Connection, Result};
use validator::Validate;

pub fn save(mut new_event: Event, conn: &Connection) -> Result<Event, String> {
    new_event
        .validate()
        .map_err(|e| format!("Event data validation error: {}", e))?;

    // Build query dynamically depending on datetime
    let result = if let Some(datetime) = new_event.datetime {
        conn.query_one(
            "INSERT INTO events (title, description, datetime)
             VALUES (?1, ?2, ?3) RETURNING id",
            (&new_event.title, &new_event.description, &datetime),
            |row| row.get(0),
        )
    } else {
        // Let SQLite use DEFAULT CURRENT_TIMESTAMP
        conn.query_one(
            "INSERT INTO events (title, description)
             VALUES (?1, ?2) RETURNING id",
            (&new_event.title, &new_event.description),
            |row| row.get(0),
        )
    };

    match result {
        Ok(id) => {
            new_event.id = id;
            Ok(new_event)
        }
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            Err(format!("A database error occurred while saving the event: {}", e))
        }
        Err(e) => Err(format!("Failed to save the new event: {}", e)),
    }}

pub fn get_all(conn: &Connection) -> Result<Vec<Event>, String> {
    let mut stmt = conn
        .prepare("SELECT * FROM events")
        .map_err(|e| format!("Failed to prepare statement: {}", e))?;

    let events_iter = stmt
        .query_map([], |row| {
            Ok(Event {
                id: row.get("id")?,
                title: row.get("title")?,
                description: row.get("description")?,
                datetime: row.get("datetime")?,
            })
        })
        .map_err(|e| format!("Failed to get events: {}", e))?;

    let events: Result<Vec<Event>, _> = events_iter.collect();
    events.map_err(|e| format!("Failed to collect events: {}", e))
}
pub fn update(event: Event, conn: &Connection) -> Result<Event, String> {
    event
        .validate()
        .map_err(|e| format!("Event data validation error: {}", e))?;

    // Build query dynamically depending on datetime
    let result = if let Some(datetime) = event.datetime {
        conn.execute(
            "UPDATE events
             SET title = ?1, description = ?2, datetime = ?3
             WHERE id = ?4",
            (
                &event.title,
                &event.description,
                &datetime,
                &event.id,
            ),
        )
    } else {
        // Skip updating datetime if None
        conn.execute(
            "UPDATE events
             SET title = ?1, description = ?2
             WHERE id = ?3",
            (
                &event.title,
                &event.description,
                &event.id,
            ),
        )
    };

    match result {
        Ok(_) => Ok(event),
        Err(rusqlite::Error::SqliteFailure(e, _)) => {
            Err(format!("A database error occurred while updating the event: {}", e))
        }
        Err(e) => Err(format!("Failed to update the event: {}", e)),
    }
}

