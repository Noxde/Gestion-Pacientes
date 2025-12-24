use crate::custom_types::structs::{Doc, Visit};
use crate::db::{visit, patient};
use crate::tests::patient::{sample_patient, another_sample_patient};

use rusqlite::Connection;
use std::{fs, path::PathBuf, fs::canonicalize, fs::read_dir};
use serial_test::serial;
use rand::prelude::*;
use chrono::{NaiveDate, NaiveDateTime, DateTime};


pub fn setup_test_db() -> (Connection, PathBuf) {
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
    fs::create_dir_all(&docs_dir).unwrap();

    patient::save(sample_patient(), &conn).expect("Failed to save patient");
    patient::save(another_sample_patient(), &conn).expect("Failed to save patient");

    // Restore .gitkeep
    let gitkeep = docs_dir.join(".gitkeep");
    fs::write(gitkeep, vec![]).expect("Failed to restore docs/.gitkeep");

    (conn, data_dir)
}

pub fn sample_visit() -> Visit {
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

pub fn another_sample_visit() -> Visit {
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

pub fn get_random_visits(amount: usize, patient_id: i32) -> Vec<Visit> {
    let mut rng = rand::rng();

    let data_dir = canonicalize("src/tests/test_data")
        .expect("Failed to get absolute path");

    let files: Vec<String> = read_dir(data_dir)
        .expect("Failed to read test_data dir")
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .map(|e| e.path().to_string_lossy().into_owned())
        .collect();


    let titles = vec![
        "Routine Check",
        "MRI",
        "Blood Test",
        "Follow-up appointment scheduled after multiple diagnostic procedures and specialist evaluations",
        "Emergency Visit at Downtown Medical Center",
        "X-Ray",
    ];

    let reasons = vec![
        "Back pain",
        "Headache",
        "Kidney issues requiring extended evaluation",
        "Routine control",
        "Post-surgery check after lumbar operation with persistent discomfort and limited mobility",
    ];

    let diagnoses = vec![
        "Herniated disc in lower lumbar region",
        "Hypertension",
        "Migraine symptoms consistent with prolonged neurological stress and environmental triggers",
        "Normal",
    ];

    let treatments = vec![
        "Physiotherapy",
        "Medication involving multiple prescriptions adjusted over several weeks of observation",
        "Extended rest with limited physical activity",
        "Surgery",
    ];

    let notes = vec![
        "Physiotherapy",
        "Medication",
        "Patient advised to rest and avoid strenuous activities",
        "Surgery performed successfully with no complications observed during the recovery period",
    ];

    let mut visits = Vec::with_capacity(amount);

    for _ in 0..amount {
        let f_amount: usize = rng.random_range(1..=files.len());

        let id: i32 = rng.random();

        let title = titles.choose(&mut rng).unwrap().to_string();

        let mut get_random = |v: Vec<&str>| {
            let return_none = &rng.random::<bool>();
            if *return_none {
                return None;
            }
            Some(v.choose(&mut rng).unwrap().to_string())
        };

        let reason = get_random(reasons.clone());
        let diagnosis = get_random(diagnoses.clone());
        let treatment = get_random(treatments.clone());
        let notes = get_random(notes.clone());

        let mut docs = Vec::with_capacity(f_amount);
        let paths: Vec<&String> = files.choose_multiple(&mut rng, f_amount).collect();
        for i in 0..f_amount {

            docs.push(Doc {
                name: String::new(),
                path: paths[i].to_owned(),
            });

        }

        let datetime = if rng.random_bool(0.5) {
            Some(DateTime::from_timestamp_nanos(rng.random::<i64>()).naive_utc())
        } else {
            None
        };

        visits.push(Visit {
            id,
            patient_id,
            title,
            reason,
            diagnosis,
            treatment,
            notes,
            docs,
            datetime,
        });
    }

    visits
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
    let result = visit::save(bad_visit.clone(), &data_dir, &mut conn);
    assert!(result.is_err());
    assert!(result.err().unwrap().contains("Title cannot be empty"));

    // Invalid file type
    bad_visit.title = "valid title".to_string();
    let bad_doc = Doc {
        name: String::new(),
        path: String::from("invalid.extension"),
    };
    bad_visit.docs.push(bad_doc);
    let result = visit::save(bad_visit.clone(), &data_dir, &mut conn);
    assert_eq!(result.err().unwrap(), "Unsupported file type: invalid.extension");

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

