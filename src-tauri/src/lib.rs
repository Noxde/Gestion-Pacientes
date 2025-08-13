use rusqlite::{Connection, Result};
use std::sync::Mutex;
use crate::db::init_db;
use tauri::{State, Manager};

mod db;

struct DbConn {
    conn: Mutex<Option<Connection>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbConn { conn: Mutex::new(None) })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, start_db])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn start_db(app: tauri::AppHandle, state: State<DbConn>) -> Result<(), String> {
    let app_data_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("Cannot find app data directory: {}", e.to_string()))?;

    let conn = init_db(app_data_dir)?;

    let mut lock = state.conn.lock().unwrap();
    *lock = Some(conn);

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
