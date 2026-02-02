mod ble;
mod parsers;
mod storage;
mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;
use commands::AppState;
use ble::BleManager;
use parsers::BvpParser;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize app state
    let app_state = AppState {
        ble_manager: Arc::new(Mutex::new(BleManager::new())),
        csv_writer: Arc::new(Mutex::new(None)),
        bvp_parser: Arc::new(Mutex::new(BvpParser::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::connect_device,
            commands::disconnect_device,
            commands::is_connected,
            commands::start_session,
            commands::stop_session,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}