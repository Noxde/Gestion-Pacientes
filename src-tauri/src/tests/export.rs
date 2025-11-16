use crate::custom_types::structs::Patient;
use crate::db::{visit, patient};
use crate::tests::patient::{sample_patient, another_sample_patient};
use crate::export;

use rusqlite::fallible_iterator::empty;
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


#[test]
#[ignore]
fn test_generate_pdf() {
    let (conn, data_dir) = setup_test_db();
    let conn = conn;

    let patients = patient::get_all(&conn).expect("Should get patients");
    if patients.is_empty() {
        panic!("Can't generate pdf, there are no patients!");
    }

    let doc = export::generate_pdf(patients[0].id, &data_dir, &conn, None).unwrap();

    println!("PDF generated successfully. Path: {}", doc.path);
}

