use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Patient {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub national_id: String,
    pub phone: String,
    pub medicare: Option<String>,
    pub medicare_number: Option<String>,
    pub sex: String,
    pub genre: String,
}
