use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Patient {
    pub id: i32,
    pub name: String,
}
