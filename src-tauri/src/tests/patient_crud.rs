#[cfg(test)]
mod tests {
    use crate::db::*;
    use crate::structs::*;
    use rusqlite::Connection;
    use std::{fs, path::PathBuf};

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

    fn sample_patient() -> Patient {
        Patient {
            id: 0,
            name: "Gregory".to_string(),
            surname: "House".to_string(),
            national_id: "12345678".to_string(),
            phone: "555-1234".to_string(),
            medicare: Some("PlanA".to_string()),
            medicare_number: Some("987654".to_string()),
            sex: "M".to_string(),
            gender: "Male".to_string(),
            description: Some("Lupus".to_string()),
        }
    }

    #[test]
    fn test_save_patient_success() {
        let conn = setup_test_db();
        let patient = sample_patient();

        save_patient(patient, &conn).expect("Should save patient");
        let patients = get_patients(&conn).expect("Should get patients");

        assert_eq!(patients.len(), 1);
        assert_eq!(patients[0].name, "Gregory");
        assert_eq!(patients[0].medicare, Some("PlanA".to_string()));
        assert_eq!(patients[0].description, Some("Lupus".to_string()));
    }

    #[test]
    fn test_save_patient_failure_for_existing_national_id() {
        let conn = setup_test_db();
        let patient = sample_patient();

        save_patient(patient, &conn).expect("Should save patient");
        let duplicate = sample_patient();

        let result = save_patient(duplicate, &conn);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            "A patient with this National ID already exists."
        );
    }

    #[test]
    fn test_get_patients_empty() {
        let conn = setup_test_db();

        let patients = get_patients(&conn).expect("Should get patients");
        assert_eq!(patients.len(), 0);
    }
}
