use crate::custom_types::enums::*;
use crate::custom_types::structs::*;
use crate::db::patient;
use rusqlite::Connection;
use std::{fs, path::PathBuf};
use serial_test::serial;

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
