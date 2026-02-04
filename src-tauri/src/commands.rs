use btleplug::api::Peripheral as _;
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ble::{BleManager, constants::*};
use crate::storage::CsvWriter;
use crate::parsers::{BvpParser, EdaParser, TempParser, AccParser};
use serde::{Serialize, Deserialize};
use tauri::{Emitter};

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub id: String,
}

#[derive(Serialize, Clone)]
pub struct SensorReading {
    pub sensor_type: String,
    pub value: serde_json::Value,
    pub timestamp: String,
}

pub struct AppState {
    pub ble_manager: Arc<Mutex<BleManager>>,
    pub csv_writer: Arc<Mutex<Option<CsvWriter>>>,
    pub bvp_parser: Arc<Mutex<BvpParser>>,
}

#[tauri::command]
pub async fn scan_for_devices() -> Result<Vec<DeviceInfo>, String> {
    use btleplug::api::{Central, Manager as _, ScanFilter};
    use btleplug::platform::Manager;
    use tokio::time::{sleep, Duration};

    // Create a temporary manager just for scanning
    let manager = Manager::new().await
        .map_err(|e| format!("Bluetooth manager error: {}", e))?;

    let adapters = manager.adapters().await
        .map_err(|e| format!("No Bluetooth adapter: {}", e))?;

    let adapter = adapters.into_iter().next()
        .ok_or("No Bluetooth adapter available")?;

    adapter.start_scan(ScanFilter::default()).await
        .map_err(|e| format!("Scan failed: {}", e))?;

    // Scan for 3 seconds
    sleep(Duration::from_secs(3)).await;

    let peripherals = adapter.peripherals().await
        .map_err(|e| format!("Failed to get devices: {}", e))?;

    let mut devices = Vec::new();

    for peripheral in peripherals {
        if let Ok(Some(props)) = peripheral.properties().await {
            if let Some(local_name) = props.local_name {
                if local_name.starts_with("Empatica E4") {
                    devices.push(DeviceInfo {
                        name: local_name.clone(),
                        id: props.address.to_string(), // this should work
                    });
                }
            }
        }
    }

    // Stop scanning to free up the adapter
    let _ = adapter.stop_scan().await;

    Ok(devices)
}

#[tauri::command]
pub async fn connect_device(
    state: tauri::State<'_, AppState>,
    id: String // ADDED: Accept ID from frontend
) -> Result<String, String> {
    let mut ble = state.ble_manager.lock().await;

    // Pass the ID to the manager
    match ble.scan_and_connect(&id).await {
        Ok(device_name) => Ok(device_name),
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[tauri::command]
pub async fn disconnect_device(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut ble = state.ble_manager.lock().await;

    match ble.disconnect().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Disconnect failed: {}", e)),
    }
}

#[tauri::command]
pub async fn is_connected(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let ble = state.ble_manager.lock().await;
    Ok(ble.is_connected())
}

#[tauri::command]
pub async fn start_streaming(
    state: tauri::State<'_, AppState>,
    app: tauri::AppHandle,
) -> Result<(), String> {
    let ble_lock = state.ble_manager.lock().await;

    let device = ble_lock.get_device()
        .ok_or("No device connected")?;

    let mut notifications = device.notifications().await
        .map_err(|e| format!("Failed to get notification stream: {}", e))?;

    let bvp_parser = state.bvp_parser.clone();
    let csv_writer = state.csv_writer.clone();
    let app_clone = app.clone();

    tokio::spawn(async move {
        while let Some(data) = notifications.next().await {
            let timestamp = chrono::Utc::now().to_rfc3339();

            match data.uuid {
                BVP_CHAR_UUID => {
                    let mut parser = bvp_parser.lock().await;
                    let readings = parser.parse(&data.value);
                    for value in readings {
                        let _ = app_clone.emit("sensor-data", SensorReading {
                            sensor_type: "bvp".to_string(),
                            value: serde_json::json!(value),
                            timestamp: timestamp.clone(),
                        });
                        if let Some(writer) = csv_writer.lock().await.as_mut() {
                            let _ = writer.write_bvp(value);
                        }
                    }
                }
                EDA_CHAR_UUID => {
                    let readings = EdaParser::parse(&data.value);
                    for value in readings {
                        let _ = app_clone.emit("sensor-data", SensorReading {
                            sensor_type: "eda".to_string(),
                            value: serde_json::json!(value),
                            timestamp: timestamp.clone(),
                        });
                        if let Some(writer) = csv_writer.lock().await.as_mut() {
                            let _ = writer.write_eda(value);
                        }
                    }
                }
                TEMP_CHAR_UUID => {
                    let readings = TempParser::parse(&data.value);
                    for value in readings {
                        let _ = app_clone.emit("sensor-data", SensorReading {
                            sensor_type: "temperature".to_string(),
                            value: serde_json::json!(value),
                            timestamp: timestamp.clone(),
                        });
                        if let Some(writer) = csv_writer.lock().await.as_mut() {
                            let _ = writer.write_temp(value);
                        }
                    }
                }
                ACC_CHAR_UUID => {
                    let readings = AccParser::parse(&data.value);
                    for acc in readings {
                        let _ = app_clone.emit("sensor-data", SensorReading {
                            sensor_type: "acc".to_string(),
                            value: serde_json::json!({"x": acc.x, "y": acc.y, "z": acc.z}),
                            timestamp: timestamp.clone(),
                        });
                        if let Some(writer) = csv_writer.lock().await.as_mut() {
                            let _ = writer.write_acc(acc.x, acc.y, acc.z);
                        }
                    }
                }
                _ => {}
            }
        }
    });

    // Send the command to the device to start sending data
    ble_lock.start_streaming().await
        .map_err(|e| format!("Failed: {}", e))?;

    Ok(())
}

#[tauri::command]
pub async fn stop_streaming(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let ble = state.ble_manager.lock().await;

    ble.stop_streaming().await
        .map_err(|e| format!("Failed to stop streaming: {}", e))?;

    Ok(())
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
pub async fn stop_session(state: tauri::State<'_, AppState>) -> Result<(), String> {
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