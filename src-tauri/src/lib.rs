use rusqlite::{Connection, Result};
use std::sync::Mutex;
use crate::db::{init_db, save_patient, get_patients};
use crate::structs::Patient;
use tauri::{State, Manager};

mod db;
mod structs;

struct DbConn {
    conn: Mutex<Option<Connection>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbConn { conn: Mutex::new(None) })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_db_comm, save_patient_comm, get_patients_comm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_db_comm(app: tauri::AppHandle, state: State<DbConn>) -> Result<(), String> {
    let app_data_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("Cannot find app data directory: {}", e.to_string()))?;

    let conn = init_db(app_data_dir)?;

    let mut lock = state.conn.lock().unwrap();
    *lock = Some(conn);

    Ok(())
}

#[tauri::command]
fn save_patient_comm(state: State<DbConn>, new_patient: Patient) -> Result<Patient, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    save_patient(new_patient, conn_ref)
}

#[tauri::command]
fn get_patients_comm(state: State<DbConn>) -> Result<Vec<Patient>, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    get_patients(conn_ref)
}
