use crate::custom_types::enums::*;
use crate::custom_types::structs::*;
use crate::db::{patient, visit};
use crate::tests::visit::{
    sample_visit,
    setup_test_db as setup_test_db_and_appdir,
    another_sample_visit
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
    conn
}

pub fn sample_patient() -> Patient {
    Patient {
        id: 1,
        name: "Gregory".to_string(),
        surname: "House".to_string(),
        national_id: "12345678".to_string(),
        phone: "555-1234".to_string(),
        medicare: Some("PlanA".to_string()),
        medicare_number: Some("987654".to_string()),
        sex: Sex::Male,
        gender: Some("Male".to_string()),
        description: Some("Lupus".to_string()),
    }
}

pub fn another_sample_patient() -> Patient {
    Patient {
        id: 2,
        name: "Eric".to_string(),
        surname: "Foreman".to_string(),
        national_id: "87654321".to_string(),
        phone: "123-5555".to_string(),
        medicare: None,
        medicare_number: None,
        sex: Sex::Male,
        gender: Some("Male".to_string()),
        description: None,
    }
}

#[test]
#[serial]
fn test_save_patient_success() {
    let conn = setup_test_db();
    let patient = sample_patient();

    patient::save(patient, &conn).expect("Should save patient");
    let patients = patient::get_all(&conn).expect("Should get patients");

    assert_eq!(patients.len(), 1);
    assert_eq!(patients[0].name, "Gregory");
    assert_eq!(patients[0].medicare, Some("PlanA".to_string()));
    assert_eq!(patients[0].description, Some("Lupus".to_string()));
}

#[test]
#[serial]
fn test_save_patient_failure_for_existing_national_id() {
    let conn = setup_test_db();
    let patient = sample_patient();

    patient::save(patient, &conn).expect("Should save patient");
    let duplicate = sample_patient();

    let result = patient::save(duplicate, &conn);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "A patient with this National ID already exists."
    );
}

#[test]
#[serial]
fn test_get_patients_empty() {
    let conn = setup_test_db();

    let patients = patient::get_all(&conn).expect("Should get patients");
    assert_eq!(patients.len(), 0);
}

#[test]
#[serial]
fn test_save_patient_failure_for_bad_request() {
    let conn = setup_test_db();
    let mut bad_patient = sample_patient();
    bad_patient.national_id = "".to_string();

    let result = patient::save(bad_patient, &conn);
    assert!(result.is_err());
    assert!(result
        .err()
        .unwrap()
        .contains("National ID cannot be empty"));
}

#[test]
#[serial]
fn test_update_patient_success() {
    let conn = setup_test_db();
    let mut patient = sample_patient();

    patient::save(patient.clone(), &conn).expect("Should save patient");
    let patients = patient::get_all(&conn).expect("Should get patients");

    assert_eq!(patients.len(), 1);
    assert_eq!(patients[0].name, "Gregory");
    assert_eq!(patients[0].phone, "555-1234");

    patient.name = "Eric".to_string();
    patient.phone = "987-5432".to_string();
    patient::update(patient, &conn).expect("Should update patient");
    let updated_patients = patient::get_all(&conn).expect("Should get patients");

    assert_eq!(updated_patients.len(), 1);
    assert_eq!(updated_patients[0].name, "Eric");
}

#[test]
#[serial]
fn test_update_patient_failure_for_bad_request() {
    let conn = setup_test_db();
    let mut patient = sample_patient();
    let another_patient = another_sample_patient();

    patient::save(patient.clone(), &conn).expect("Should save patient");
    patient::save(another_patient.clone(), &conn).expect("Should save patient");

    patient.national_id = another_patient.national_id.clone();

    let result = patient::update(patient, &conn);
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        "A patient with this National ID already exists."
    );
}

#[test]
#[serial]
fn test_patient_get() {
    let conn = setup_test_db();

    // Insert patient
    let patient = sample_patient();
    patient::save(patient.clone(), &conn).expect("Should save patient");

    // --- SUCCESS ---
    let fetched = patient::get(patient.id, &conn).expect("Should fetch patient");
    assert_eq!(fetched.id, patient.id);
    assert_eq!(fetched.name, "Gregory");
    assert_eq!(fetched.national_id, "12345678");

    // --- NOT FOUND ---
    let result = patient::get(999, &conn);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("Failed to load patient"));
}

#[test]
#[serial]
fn test_patient_get_medical_history() {
    let (conn, data_dir) = setup_test_db_and_appdir();
    let mut conn = conn; // make mutable for save() which needs &mut Connection

    // Save two visits
    visit::save(sample_visit(), &data_dir, &mut conn).expect("Should save visit");
    visit::save(another_sample_visit(), &data_dir, &mut conn).expect("Should save visit");

    // --- SUCCESS ---
    let history = patient::get_medical_history(&conn, 1, &data_dir)
        .expect("Should get medical history");

    assert_eq!(history.patient.id, 1);
    assert_eq!(history.patient.name, "Gregory");
    assert_eq!(history.patient.medicare, Some("PlanA".to_string()));
    assert_eq!(history.patient.description, Some("Lupus".to_string()));

    assert_eq!(history.visits.len(), 2);
    assert_eq!(history.visits[0].title, "Routine Check");
    assert_eq!(history.visits[0].reason, Some("Kidney issues".to_string()));
    assert!(history.visits[0].datetime.is_some());
    assert_eq!(history.visits[0].docs[0].name, "file1.jpg");
    assert_eq!(history.visits[0].docs[1].name, "file2.png");
    assert_eq!(history.visits[0].docs[2].name, "file3.pdf");

    assert_eq!(history.visits[1].title, "MRI");
    assert_eq!(history.visits[1].reason, Some("Back pain".to_string()));
    let expected_dt = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();
    assert_eq!(history.visits[1].datetime, Some(expected_dt));
    assert!(history.visits[1].docs.is_empty());

    // --- NOT FOUND ---
    let result = patient::get_medical_history(&conn, 999, &data_dir);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("Failed to load patient"));
}
