use crate::custom_types::structs::Event;
use crate::db::event;
use rusqlite::Connection;
use std::{fs, path::PathBuf};
use serial_test::serial;
use chrono::NaiveDate;

fn setup_test_db() -> Connection {
    let db_path = PathBuf::from("src/tests/test_db.sqlite");
    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }
    let conn = Connection::open(&db_path).unwrap();
    let schema = std::fs::read_to_string("schema.sql").unwrap();
    conn.execute_batch(&schema).unwrap();
    conn
}

fn sample_event() -> Event {
    Event {
        id: 1,
        title: "Rutine Check".to_string(),
        description: Some("Kidney bad".to_string()),
        datetime: None,
    }
}

fn another_sample_event() -> Event {
    Event {
        id: 2,
        title: "MRI".to_string(),
        description: None,
        datetime: Some(NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap()),
    }
}

#[test]
#[serial]
fn test_get_save_update() {
    let conn = setup_test_db();

    // Test get_all() with no events
    let events = event::get_all(&conn).expect("Should get events");
    assert_eq!(events.len(), 0);

    // Test saving an event
    let event = sample_event();
    let mut event = event::save(event, &conn).expect("Should save event");
    let events = event::get_all(&conn).expect("Should get events");

    assert_eq!(events.len(), 1);
    assert_eq!(events[0].title, "Rutine Check");
    assert_eq!(events[0].description, Some("Kidney bad".to_string()));
    assert!(events[0].datetime.is_some());

    // Test saving with no title
    let mut bad_event = sample_event();
    bad_event.title = "".to_string();

    let result = event::save(bad_event, &conn);
    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .contains("Title cannot be empty"));

    // Test update
    event.title = "Not a Rutine Check".to_string();
    event::update(event, &conn).expect("Should update event");
    let updated_events = event::get_all(&conn).expect("Should get events");

    assert_eq!(updated_events.len(), 1);
    assert_eq!(updated_events[0].title, "Not a Rutine Check");

    // Test saving an event with datetime
    let event = another_sample_event();
    event::save(event, &conn).expect("Should save event");
    let events = event::get_all(&conn).expect("Should get events");

    assert_eq!(events.len(), 2);
    assert_eq!(events[1].title, "MRI");
    assert_eq!(events[1].description, None);
    assert_eq!(events[1].datetime, Some(NaiveDate::from_ymd_opt(2016, 7, 8).unwrap().and_hms_opt(9, 10, 11).unwrap()));

}
