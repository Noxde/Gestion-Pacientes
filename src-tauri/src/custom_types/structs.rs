use crate::custom_types::enums::Sex;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Validate)]
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
