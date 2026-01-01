use crate::custom_types::structs::Patient;
use crate::db::{visit, patient};
use crate::tests::patient::*;
use crate::tests::visit::get_random_visits;
use crate::export::{self, justify};

use rusqlite::fallible_iterator::empty;
use rusqlite::Connection;
use std::{fs, path::PathBuf, fs::canonicalize};
use serial_test::serial;
use chrono::NaiveDate;
use rand::prelude::*;

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


#[test]
#[ignore]
#[serial]
fn test_generate_pdf() {
    let (conn, data_dir) = setup_test_db();
    let conn = conn;

    let patients = patient::get_all(&conn).expect("Should get patients");
    if patients.is_empty() {
        panic!("Can't generate pdf, there are no patients!");
    }

    // Test default save_path
    export::generate_pdf(patients[0].id, &data_dir, &conn, None).unwrap();

    // Create test path
    let test_ex_dir = data_dir.join("test_exports");
    if test_ex_dir.exists() {
        fs::remove_dir_all(&test_ex_dir).unwrap();
    }
    fs::create_dir_all(&test_ex_dir).unwrap();

    // Test providing a path to a folder
    let test_file_name = test_ex_dir.join("gregory_house_historia_medica.pdf");
    let test_ex_dir_string = test_ex_dir.to_string_lossy().to_string();
    // Assert the file does not exist
    assert!(!test_file_name.exists());
    // Export
    let doc = export::generate_pdf(patients[0].id, &data_dir, &conn, Some(test_ex_dir_string.clone())).unwrap();
    // The file exists
    assert!(test_file_name.exists());
    assert_eq!(doc.path, test_file_name.to_string_lossy().to_string());

    // Test providing a path to a file
    let test_file_name = test_ex_dir.join("test_name.pdf");
    let test_file_name_string = test_file_name.to_string_lossy().to_string();
    // Assert the file does not exist
    assert!(!test_file_name.exists());
    // Export
    let doc = export::generate_pdf(patients[0].id, &data_dir, &conn, Some(test_file_name_string.clone())).unwrap();
    // The file exists
    assert!(test_file_name.exists());
    assert_eq!(doc.path, test_file_name_string);

    // Test retry with folder path
    let exports_path = data_dir.join("exports");
    // Delete the previous exporst
    fs::remove_dir_all(&exports_path).unwrap();
    fs::create_dir_all(&exports_path).unwrap();
    let default_path = exports_path.join("gregory_house_historia_medica.pdf");
    assert!(!default_path.exists());
    // Export
    let invalid_path_folder = String::from("/this/is/invalid/");
    let doc = export::generate_pdf(patients[0].id, &data_dir, &conn, Some(invalid_path_folder)).unwrap();
    // The file was created in the default path
    assert!(default_path.exists());
    assert_eq!(doc.path, default_path.to_string_lossy().to_string());

    // Test retry with file name
    let exports_path = data_dir.join("exports");
    // Delete the previous exporst
    fs::remove_dir_all(&exports_path).unwrap();
    fs::create_dir_all(&exports_path).unwrap();
    let default_path = exports_path.join("gregory_house_historia_medica.pdf");
    assert!(!default_path.exists());
    // Export
    let invalid_path_folder = String::from("/this/is/invalid/filename.pdf");
    let doc = export::generate_pdf(patients[0].id, &data_dir, &conn, Some(invalid_path_folder)).unwrap();
    // The file was created in the default path
    assert!(default_path.exists());
    assert_eq!(doc.path, default_path.to_string_lossy().to_string());

    // Restore .gitkeep
    let gitkeep = exports_path.join(".gitkeep");
    fs::write(gitkeep, vec![]).expect("Failed to restore exports/.gitkeep");
}

#[test]
#[ignore]
#[serial]
fn test_generate_random_pdf() {
    let (mut conn, data_dir) = setup_test_db();

    let mut rng = rand::rng();

    let amount: usize = rng.random_range(5..=10);

    let patient = random_patient();
    let patient = patient::save(patient, &conn).expect("Should save patient");
    let visits = get_random_visits(amount, patient.id);
    for v in visits {
        visit::save(v, &data_dir, &mut conn).expect("Should save visit");
    }

    // Test default save_path
    export::generate_pdf(patient.id, &data_dir, &conn, None).unwrap();
}

#[test]
fn test_justify() {
    let text = "Paciente: Charles Alexander Long Name Francis Johnson Thompson York";
    let lines = vec![
        "Paciente:  Charles  Alexander  Long Name Francis Johnson",
        "Thompson York"
    ];
    assert_eq!(justify(text.to_string(), 56), lines);

    //If the text len < max_width, the text is returned directly
    let text = "Hello   how   are   you";
    let lines = vec![text];
    assert_eq!(justify(text.to_string(), 30), lines);

    //max_width must be > 1
    assert!(justify(text.to_string(), 1).is_empty());

    let text = "ThisIsATest";
    let lines = vec![
        "T-", "h-", "i-", "s-", "I-",
        "s-", "A-", "T-", "e-", "st"
    ];
    assert_eq!(justify(text.to_string(), 2), lines);
}
