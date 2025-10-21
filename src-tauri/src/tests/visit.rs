use crate::custom_types::structs::Visit;
use crate::db::{
    visit,
    patient,
};
use crate::tests::patient::{
    sample_patient,
    another_sample_patient,
};
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
    patient::save(sample_patient(), &conn).expect("Failed to save patient");
    patient::save(another_sample_patient(), &conn).expect("Failed to save patient");
    conn
}

fn sample_visit() -> Visit {
    Visit {
        id: 1,
        patient_id: 1,
        title: "Routine Check".to_string(),
        reason: Some("Kidney issues".to_string()),
        diagnosis: None,
        treatment: None,
        notes: None,
        files: vec![],
        datetime: None,
    }
}

fn another_sample_visit() -> Visit {
    Visit {
        id: 2,
        patient_id: 1,
        title: "MRI".to_string(),
        reason: Some("Back pain".to_string()),
        diagnosis: Some("Herniated disc".to_string()),
        treatment: Some("Physiotherapy".to_string()),
        notes: None,
        files: vec![],
        datetime: Some(
            NaiveDate::from_ymd_opt(2016, 7, 8)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap(),
        ),
    }
}

fn visit_patient_2() -> Visit {
    Visit {
        id: 1,
        patient_id: 2,
        title: "Routine Check".to_string(),
        reason: Some("Follow-up visit".to_string()),
        diagnosis: None,
        treatment: None,
        notes: None,
        files: vec![],
        datetime: None,
    }
}

#[test]
#[serial]
fn test_get_save_update() {
    let conn = setup_test_db();

    // Test get_all() with no visits
    let visits = visit::get_all(1, &conn).expect("Should get visits");
    assert_eq!(visits.len(), 0);

    // Test saving two visits
    let visit = sample_visit();
    let mut visit = visit::save(visit, 1, &conn).expect("Should save visit");
    visit::save(visit_patient_2(), 2, &conn).expect("Should save visit");
    let visits = visit::get_all(1, &conn).expect("Should get visits");

    assert_eq!(visits.len(), 1);
    assert_eq!(visits[0].title, "Routine Check");
    assert_eq!(visits[0].reason, Some("Kidney issues".to_string()));
    assert!(visits[0].datetime.is_some());

    // Test saving with no title
    let mut bad_visit = sample_visit();
    bad_visit.title = "".to_string();

    let result = visit::save(bad_visit, 1, &conn);
    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .contains("Title cannot be empty"));

    // Test update
    visit.title = "Not a Routine Check".to_string();
    visit::update(visit, &conn).expect("Should update visit");
    let updated_visits = visit::get_all(1, &conn).expect("Should get visits");

    assert_eq!(updated_visits.len(), 1);
    assert_eq!(updated_visits[0].title, "Not a Routine Check");

    // Test saving a visit with datetime
    let visit = another_sample_visit();
    visit::save(visit, 1, &conn).expect("Should save visit");
    let visits = visit::get_all(1, &conn).expect("Should get visits");

    assert_eq!(visits.len(), 2);
    assert_eq!(visits[1].title, "MRI");
    assert_eq!(visits[1].reason, Some("Back pain".to_string()));
    assert_eq!(
        visits[1].datetime,
        Some(
            NaiveDate::from_ymd_opt(2016, 7, 8)
                .unwrap()
                .and_hms_opt(9, 10, 11)
                .unwrap()
        )
    );

    // Try to save a visit with invalid patient id
    let visit = sample_visit();
    let result = visit::save(visit, i32::MAX, &conn);
    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .contains("Invalid patient_id"));
}
