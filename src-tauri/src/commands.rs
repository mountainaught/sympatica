use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ble::BleManager;
use crate::storage::CsvWriter;
use crate::parsers::{BvpParser, EdaParser, TempParser, AccParser};

// App state that will be shared across commands
pub struct AppState {
    pub ble_manager: Arc<Mutex<BleManager>>,
    pub csv_writer: Arc<Mutex<Option<CsvWriter>>>,
    pub bvp_parser: Arc<Mutex<BvpParser>>,
}

#[tauri::command]
pub async fn connect_device(
    state: tauri::State<'_, AppState>,
) -> Result<String, String> {
    let mut ble = state.ble_manager.lock().await;

    match ble.scan_and_connect().await {
        Ok(device_name) => Ok(device_name),
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[tauri::command]
pub async fn disconnect_device(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut ble = state.ble_manager.lock().await;

    match ble.disconnect().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Disconnect failed: {}", e)),
    }
}

#[tauri::command]
pub async fn is_connected(
    state: tauri::State<'_, AppState>,
) -> Result<bool, String> {
    let ble = state.ble_manager.lock().await;
    Ok(ble.is_connected())
}

#[tauri::command]
pub async fn start_session(
    state: tauri::State<'_, AppState>,
    base_path: String,
    session_name: String,
) -> Result<(), String> {
    let mut writer_lock = state.csv_writer.lock().await;

    match CsvWriter::new(&base_path, &session_name) {
        Ok(writer) => {
            *writer_lock = Some(writer);
            Ok(())
        }
        Err(e) => Err(format!("Failed to create session: {}", e)),
    }
}

#[tauri::command]
pub async fn stop_session(
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let mut writer_lock = state.csv_writer.lock().await;

    if let Some(mut writer) = writer_lock.take() {
        match writer.close() {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Failed to close session: {}", e)),
        }
    } else {
        Ok(())
    }
}

// TODO: We'll add start_reading and the actual data streaming later
// This requires more complex async channels and event emission to frontend