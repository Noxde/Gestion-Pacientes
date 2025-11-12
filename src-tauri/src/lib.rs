use crate::custom_types::structs::{
    Patient,
    Visit,
    Doc
};
use crate::db::{
    common::init_db,
    patient,
    visit
};
use rusqlite::{Connection, Result};
use std::{
    sync::Mutex,
    path::PathBuf,
};
use tauri::{Manager, State};

mod custom_types;
mod db;
mod tests;
mod macros;
mod export;

struct AppData {
    conn: Mutex<Option<Connection>>,
    data_dir: Mutex<Option<PathBuf>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(AppData {
            conn: Mutex::new(None),
            data_dir: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            init_db_comm,
            save_patient_comm,
            get_patients_comm,
            update_patient_comm,
            save_visit_comm,
            get_visits_comm,
            update_visit_comm,
            export_to_pdf_comm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn init_db_comm(app: tauri::AppHandle, state: State<AppData>) -> Result<(), String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| format!("Cannot find app data directory: {}", e.to_string()))?;

    let conn = log_err!(init_db(&app_data_dir))?;

    let mut lock = state.conn.lock().unwrap();
    *lock = Some(conn);

    let mut lock = state.data_dir.lock().unwrap();
    *lock = Some(app_data_dir);

    Ok(())
}

#[tauri::command]
fn save_patient_comm(state: State<AppData>, new_patient: Patient) -> Result<Patient, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    log_err!(patient::save(new_patient, conn_ref))
}

#[tauri::command]
fn get_patients_comm(state: State<AppData>) -> Result<Vec<Patient>, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    log_err!(patient::get_all(conn_ref))
}

#[tauri::command]
fn update_patient_comm(state: State<AppData>, patient: Patient) -> Result<Patient, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    log_err!(patient::update(patient, conn_ref))
}

#[tauri::command]
fn save_visit_comm(state: State<AppData>, new_visit: Visit) -> Result<Visit, String> {
    let mut conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_mut().ok_or("Database not initialized")?;

    let data_dir = state.data_dir.lock().unwrap();
    let data_dir_ref = data_dir.as_ref().ok_or("Database not initialized")?;
    log_err!(visit::save(new_visit, data_dir_ref, conn_ref))
}

#[tauri::command]
fn get_visits_comm(patient_id: i32, state: State<AppData>) -> Result<Vec<Visit>, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;

    let data_dir = state.data_dir.lock().unwrap();
    let data_dir_ref = data_dir.as_ref().ok_or("Database not initialized")?;
    log_err!(visit::get_all(patient_id, data_dir_ref, conn_ref))
}

#[tauri::command]
fn update_visit_comm(state: State<AppData>, visit: Visit) -> Result<Visit, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    log_err!(visit::update(visit, conn_ref))
}

#[tauri::command]
fn export_to_pdf_comm(state: State<AppData>, patient_id: i32) -> Result<Doc, String> {
    let conn = state.conn.lock().unwrap();
    let conn_ref = conn.as_ref().ok_or("Database not initialized")?;
    log_err!(export::generate_pdf(conn_ref, patient_id))
}
