use crate::custom_types::enums::Sex;
use serde::{Deserialize, Serialize};
use validator::Validate;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Patient {
    pub id: i32,
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Surname cannot be empty"))]
    pub surname: String,
    #[validate(length(min = 1, message = "National ID cannot be empty"))]
    pub national_id: String,
    #[validate(length(min = 1, message = "Phone cannot be empty"))]
    pub phone: String,
    pub medicare: Option<String>,
    pub medicare_number: Option<String>,
    pub sex: Sex,
    pub gender: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Visit {
    pub id: i32,
    pub patient_id: i32,
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    pub reason: Option<String>,
    pub diagnosis: Option<String>,
    pub treatment: Option<String>,
    pub notes: Option<String>,
    pub docs: Vec<Doc>,
    pub datetime: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct Doc {
    pub path: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Validate, Clone)]
pub struct MedicalHistory {
    pub patient: Patient,
    pub visits: Vec<Visit>,
}
