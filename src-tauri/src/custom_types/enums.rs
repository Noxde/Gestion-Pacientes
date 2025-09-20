use rusqlite::types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Sex {
    Male,
    Female,
    Other,
}

impl FromSql for Sex {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "M" => Ok(Sex::Male),
            "F" => Ok(Sex::Female),
            "-" => Ok(Sex::Other),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Sex {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Sex::Male => Ok("M".into()),
            Sex::Female => Ok("F".into()),
            Sex::Other => Ok("-".into()),
        }
    }
}
