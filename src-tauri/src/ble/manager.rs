use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, WriteType, Characteristic,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use crate::ble::constants::*;

pub struct BleManager {
    adapter: Option<Adapter>,
    device: Option<Peripheral>,
    characteristics: Option<DeviceCharacteristics>,
}

pub struct DeviceCharacteristics {
    pub bvp: Characteristic,
    pub eda: Characteristic,
    pub temp: Characteristic,
    pub acc: Characteristic,
    pub cmd: Characteristic,
}

impl BleManager {
    pub fn new() -> Self {
        Self {
            adapter: None,
            device: None,
            characteristics: None,
        }
    }

    // Helper to reuse the adapter connection
    async fn get_adapter(&mut self) -> Result<Adapter, Box<dyn Error>> {
        if let Some(adapter) = &self.adapter {
            return Ok(adapter.clone());
        }

        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let adapter = adapters.into_iter().next()
            .ok_or("No Bluetooth adapter found")?;

        self.adapter = Some(adapter.clone());
        Ok(adapter)
    }

    // CHANGED: Now accepts `device_id` to ensure we connect to the right device
    pub async fn scan_and_connect(&mut self, device_id: &str) -> Result<String, Box<dyn Error>> {
        let adapter = self.get_adapter().await?;
        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(2)).await;

        let peripherals = adapter.peripherals().await?;

        // Find device by matching address from properties
        let mut target_device = None;
        for p in peripherals {
            if let Ok(Some(props)) = p.properties().await {
                if props.address.to_string() == device_id {
                    target_device = Some(p);
                    break;
                }
            }
        }

        let device = target_device.ok_or(format!("Device {} not found", device_id))?;

        let _ = adapter.stop_scan().await;

        // Connect
        device.connect().await?;
        device.discover_services().await?;

        let services = device.services();
        let sensor_service = services.iter()
            .find(|s| s.uuid == SENSOR_SERVICE_UUID)
            .ok_or("Sensor service not found")?;
        let cmd_service = services.iter()
            .find(|s| s.uuid == CMD_SERVICE_UUID)
            .ok_or("Command service not found")?;

        let bvp = sensor_service.characteristics.iter()
            .find(|c| c.uuid == BVP_CHAR_UUID)
            .ok_or("BVP characteristic not found")?
            .clone();
        let eda = sensor_service.characteristics.iter()
            .find(|c| c.uuid == EDA_CHAR_UUID)
            .ok_or("EDA characteristic not found")?
            .clone();
        let temp = sensor_service.characteristics.iter()
            .find(|c| c.uuid == TEMP_CHAR_UUID)
            .ok_or("TEMP characteristic not found")?
            .clone();
        let acc = sensor_service.characteristics.iter()
            .find(|c| c.uuid == ACC_CHAR_UUID)
            .ok_or("ACC characteristic not found")?
            .clone();
        let cmd = cmd_service.characteristics.iter()
            .find(|c| c.uuid == CMD_CHAR_UUID)
            .ok_or("CMD characteristic not found")?
            .clone();

        self.characteristics = Some(DeviceCharacteristics {
            bvp, eda, temp, acc, cmd
        });

        self.device = Some(device.clone());

        // Get the name for return
        let props = device.properties().await?;
        let local_name = props
            .and_then(|p| p.local_name)
            .unwrap_or_else(|| "Empatica E4".to_string());

        Ok(local_name)
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(device) = &self.device {
            device.disconnect().await?;
        }
        self.device = None;
        self.characteristics = None;
        Ok(())
    }

    pub fn get_device(&self) -> Option<Peripheral> {
        self.device.clone()
    }

    pub async fn start_streaming(&self) -> Result<(), Box<dyn Error>> {
        let device = self.device.as_ref().ok_or("Device not connected")?;
        let chars = self.characteristics.as_ref().ok_or("Characteristics not initialized")?;

        device.subscribe(&chars.bvp).await?;
        device.subscribe(&chars.eda).await?;
        device.subscribe(&chars.temp).await?;
        device.subscribe(&chars.acc).await?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as u32; // Changed from f64/default to u32 for E4 protocol

        // E4 protocol: 0x01 followed by 4-byte timestamp
        let mut command = vec![0x01];
        command.extend_from_slice(&timestamp.to_le_bytes());

        device.write(&chars.cmd, &command, WriteType::WithoutResponse).await?;

        Ok(())
    }

    pub async fn stop_streaming(&self) -> Result<(), Box<dyn Error>> {
        let device = self.device.as_ref().ok_or("Device not connected")?;
        let chars = self.characteristics.as_ref().ok_or("Characteristics not initialized")?;

        // Best effort unsubscribe
        let _ = device.unsubscribe(&chars.bvp).await;
        let _ = device.unsubscribe(&chars.eda).await;
        let _ = device.unsubscribe(&chars.temp).await;
        let _ = device.unsubscribe(&chars.acc).await;

        Ok(())
    }
}