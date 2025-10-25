use crate::custom_types::structs::{Doc, Visit};
use crate::db::{visit, patient};
use crate::tests::patient::{sample_patient, another_sample_patient};

use rusqlite::Connection;
use std::{fs, path::PathBuf, fs::canonicalize};
use serial_test::serial;
use chrono::NaiveDate;

fn setup_test_db() -> (Connection, PathBuf) {
    let db_path = PathBuf::from("src/tests/test_db.sqlite");
    if db_path.exists() {
        fs::remove_file(&db_path).unwrap();
    }

    let conn = Connection::open(&db_path).unwrap();
    let schema = std::fs::read_to_string("schema.sql").unwrap();
    conn.execute_batch(&schema).unwrap();

    // Ensure docs dir exists
    let data_dir = PathBuf::from("src/tests/test_data");
    let docs_dir = data_dir.join("docs");
    if docs_dir.exists() {
        fs::remove_dir_all(&docs_dir).unwrap();
    }
    fs::create_dir_all(data_dir.join("docs")).unwrap();

    patient::save(sample_patient(), &conn).expect("Failed to save patient");
    patient::save(another_sample_patient(), &conn).expect("Failed to save patient");

    (conn, data_dir)
}

fn sample_visit() -> Visit {
    let data_dir = canonicalize("src/tests/test_data")
        .expect("Failed to get absolute path");
    let file1 = Doc {
        name: String::new(),
        path: data_dir.join("file1.jpg").to_string_lossy().into_owned(),
    };

    let file2 = Doc {
        name: String::new(),
        path: data_dir.join("file2.png").to_string_lossy().into_owned(),
    };

    let file3 = Doc {
        name: String::new(),
        path: data_dir.join("file3.pdf").to_string_lossy().into_owned(),
    };
    Visit {
        id: 1,
        patient_id: 1,
        title: "Routine Check".to_string(),
        reason: Some("Kidney issues".to_string()),
        diagnosis: None,
        treatment: None,
        notes: None,
        docs: vec![file1, file2, file3],
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
        docs: vec![],
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
        docs: vec![],
        datetime: None,
    }
}

#[test]
#[serial]
fn test_get_save_update() {
    let (conn, data_dir) = setup_test_db();
    let mut conn = conn; // make mutable for save() which needs &mut Connection

    // Test get_all() with no visits
    let visits = visit::get_all(1, &data_dir, &conn).expect("Should get visits");
    assert_eq!(visits.len(), 0);

    // Save two visits
    let v = sample_visit();
    let mut v = visit::save(v, &data_dir, &mut conn).expect("Should save visit");

    visit::save(visit_patient_2(), &data_dir, &mut conn).expect("Should save visit");
    let visits = visit::get_all(1, &data_dir, &conn).expect("Should get visits");

    assert_eq!(visits.len(), 1);
    assert_eq!(visits[0].title, "Routine Check");
    assert_eq!(visits[0].reason, Some("Kidney issues".to_string()));
    assert!(visits[0].datetime.is_some());
    assert_eq!(visits[0].docs[0].name, "file1.jpg");
    assert_eq!(visits[0].docs[1].name, "file2.png");
    assert_eq!(visits[0].docs[2].name, "file3.pdf");

    // Saving with invalid title
    let mut bad_visit = sample_visit();
    bad_visit.title = "".to_string();
    let result = visit::save(bad_visit, &data_dir, &mut conn);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("Title cannot be empty"));

    // Update
    v.title = "Not a Routine Check".to_string();
    visit::update(v.clone(), &conn).expect("Should update visit");

    let updated_visits = visit::get_all(1, &data_dir, &conn).expect("Should get visits");
    assert_eq!(updated_visits.len(), 1);
    assert_eq!(updated_visits[0].title, "Not a Routine Check");

    // Save visit with datetime
    let v2 = another_sample_visit();
    visit::save(v2, &data_dir, &mut conn).expect("Should save visit");
    let visits = visit::get_all(1, &data_dir, &conn).expect("Should get visits");

    assert_eq!(visits.len(), 2);
    assert_eq!(visits[1].title, "MRI");
    assert_eq!(visits[1].reason, Some("Back pain".to_string()));

    let expected_dt = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    assert_eq!(visits[1].datetime, Some(expected_dt));

    // Invalid patient id → should error
    let mut invalid_visit = sample_visit();
    invalid_visit.patient_id = i32::MAX;
    let result = visit::save(invalid_visit, &data_dir, &mut conn);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("Invalid patient_id"));
}

