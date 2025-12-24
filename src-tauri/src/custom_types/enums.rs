use rusqlite::types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FileType {
    PDF,
    MP4,
    JPG,
    TXT,
    PNG,
    DOCX,
}

impl FileType {
    /// Convert enum to MIME type
    pub fn to_mime(&self) -> &'static str {
        match self {
            FileType::PDF  => "application/pdf",
            FileType::MP4  => "video/mp4",
            FileType::JPG  => "image/jpeg",
            FileType::TXT  => "text/plain",
            FileType::PNG  => "image/png",
            FileType::DOCX => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        }
    }
}

/// Convert a file path or extension to FileType
impl FromStr for FileType {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let path = Path::new(input);

        // Extract extension
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or(input)  // fallback to raw input
            .to_lowercase();

        match ext.as_str() {
            "pdf" => Ok(FileType::PDF),
            "mp4" => Ok(FileType::MP4),
            "jpg" | "jpeg" => Ok(FileType::JPG),
            "txt" => Ok(FileType::TXT),
            "png" => Ok(FileType::PNG),
            "docx" => Ok(FileType::DOCX),

            _ => Err(format!("Unsupported file type: {}", input)),
        }
    }
}
