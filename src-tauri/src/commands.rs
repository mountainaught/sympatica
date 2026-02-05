// src/commands.rs
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::ble::BleManager;
use crate::storage::CsvWriter;
use crate::parsers::{BvpParser, EdaParser, TempParser, AccParser};
use serde::{Serialize, Deserialize};
use tauri::Emitter;

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
    use bluest::Adapter;
    use futures::StreamExt;
    use tokio::time::{sleep, Duration};

    let adapter = Adapter::default().await
        .ok_or("No Bluetooth adapter found")?;

    adapter.wait_available().await
        .map_err(|e| format!("Bluetooth not available: {}", e))?;

    let mut scan = adapter.scan(&[]).await
        .map_err(|e| format!("Scan failed: {}", e))?;

    let mut devices = Vec::new();
    let timeout = sleep(Duration::from_secs(3));
    tokio::pin!(timeout);

    loop {
        tokio::select! {
            Some(discovered) = scan.next() => {
                if let Ok(name) = discovered.device.name() {
                    if name.starts_with("Empatica E4") {
                        let id = discovered.device.id().to_string();
                        devices.push(DeviceInfo {
                            name,
                            id,
                        });
                    }
                }
            }
            _ = &mut timeout => {
                break;
            }
        }
    }

    Ok(devices)
}

#[tauri::command]
pub async fn connect_device(
    state: tauri::State<'_, AppState>,
    id: String,
) -> Result<String, String> {
    let mut ble = state.ble_manager.lock().await;

    match ble.scan_and_connect(&id).await {
        Ok(device_name) => Ok(device_name),
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[tauri::command]
pub async fn disconnect_device(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut ble = state.ble_manager.lock().await;
    ble.disconnect().await
        .map_err(|e| format!("Disconnect failed: {}", e))
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
    println!("[START] Called");

    let chars = {
        let ble = state.ble_manager.lock().await;
        ble.get_characteristics().ok_or("No characteristics")?
    };

    let bvp_parser = state.bvp_parser.clone();
    let csv = state.csv_writer.clone();

    // BVP
    let bvp_char = chars.bvp.clone();
    let csv_bvp = csv.clone();
    let app_bvp = app.clone();
    tokio::spawn(async move {
        if let Ok(mut stream) = bvp_char.notify().await {
            let mut parser = bvp_parser.lock().await;
            while let Some(Ok(data)) = stream.next().await {
                println!("[BVP] {} bytes", data.len());
                for value in parser.parse(&data) {
                    let _ = app_bvp.emit("sensor-data", SensorReading {
                        sensor_type: "bvp".to_string(),
                        value: serde_json::json!(value),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                    if let Some(w) = csv_bvp.lock().await.as_mut() {
                        let _ = w.write_bvp(value);
                    }
                }
            }
        }
    });

    // EDA
    let eda_char = chars.eda.clone();
    let csv_eda = csv.clone();
    let app_eda = app.clone();
    tokio::spawn(async move {
        if let Ok(mut stream) = eda_char.notify().await {
            while let Some(Ok(data)) = stream.next().await {
                println!("[EDA] {} bytes", data.len());
                for value in EdaParser::parse(&data) {
                    let _ = app_eda.emit("sensor-data", SensorReading {
                        sensor_type: "eda".to_string(),
                        value: serde_json::json!(value),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                    if let Some(w) = csv_eda.lock().await.as_mut() {
                        let _ = w.write_eda(value);
                    }
                }
            }
        }
    });

    // TEMP
    let temp_char = chars.temp.clone();
    let csv_temp = csv.clone();
    let app_temp = app.clone();
    tokio::spawn(async move {
        if let Ok(mut stream) = temp_char.notify().await {
            while let Some(Ok(data)) = stream.next().await {
                println!("[TEMP] {} bytes", data.len());
                for value in TempParser::parse(&data) {
                    let _ = app_temp.emit("sensor-data", SensorReading {
                        sensor_type: "temperature".to_string(),
                        value: serde_json::json!(value),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                    if let Some(w) = csv_temp.lock().await.as_mut() {
                        let _ = w.write_temp(value);
                    }
                }
            }
        }
    });

    // ACC
    let acc_char = chars.acc.clone();
    let csv_acc = csv.clone();
    let app_acc = app.clone();
    tokio::spawn(async move {
        if let Ok(mut stream) = acc_char.notify().await {
            while let Some(Ok(data)) = stream.next().await {
                println!("[ACC] {} bytes", data.len());
                for acc in AccParser::parse(&data) {
                    let _ = app_acc.emit("sensor-data", SensorReading {
                        sensor_type: "acc".to_string(),
                        value: serde_json::json!({"x": acc.x, "y": acc.y, "z": acc.z}),
                        timestamp: chrono::Utc::now().to_rfc3339(),
                    });
                    if let Some(w) = csv_acc.lock().await.as_mut() {
                        let _ = w.write_acc(acc.x, acc.y, acc.z);
                    }
                }
            }
        }
    });

    // Give tasks a moment to start
    // Give tasks a moment to start
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    // Send command
    let ble = state.ble_manager.lock().await;
    ble.start_streaming().await.map_err(|e| e.to_string())?;
    println!("[START] Command sent");

    Ok(())
}

#[tauri::command]
pub async fn stop_streaming(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let ble = state.ble_manager.lock().await;
    ble.stop_streaming().await
        .map_err(|e| format!("Failed to stop streaming: {}", e))
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