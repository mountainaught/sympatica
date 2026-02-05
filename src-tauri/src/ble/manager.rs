// src/ble/manager.rs
use bluest::{Adapter, Device, Characteristic};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use crate::ble::constants::*;
use futures::StreamExt;

pub struct BleManager {
    adapter: Option<Adapter>,
    device: Option<Device>,
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

    async fn get_adapter(&mut self) -> Result<Adapter, Box<dyn Error>> {
        if let Some(adapter) = &self.adapter {
            return Ok(adapter.clone());
        }

        let adapter = Adapter::default().await
            .ok_or("No Bluetooth adapter found")?;

        adapter.wait_available().await?;

        self.adapter = Some(adapter.clone());
        Ok(adapter)
    }

    pub async fn scan_and_connect(&mut self, device_id: &str) -> Result<String, Box<dyn Error>> {
        let adapter = self.get_adapter().await?;

        let mut scan = adapter.scan(&[]).await?;
        let mut target_device = None;

        let timeout = time::sleep(Duration::from_secs(5));
        tokio::pin!(timeout);

        loop {
            tokio::select! {
                Some(discovered) = scan.next() => {
                    let addr_str = discovered.device.id().to_string();
                    if addr_str == device_id {
                        target_device = Some(discovered.device);
                        break;
                    }
                }
                _ = &mut timeout => {
                    break;
                }
            }
        }

        let device = target_device.ok_or(format!("Device {} not found during scan", device_id))?;

        // Connect via adapter, not device
        adapter.connect_device(&device).await?;

        time::sleep(Duration::from_millis(500)).await;

        // Discover services
        let services = device.discover_services().await?;

        // Find sensor service
        let sensor_service = services.iter()
            .find(|s| s.uuid() == SENSOR_SERVICE_UUID)
            .ok_or("Sensor service not found")?;

        // Find command service
        let cmd_service = services.iter()
            .find(|s| s.uuid() == CMD_SERVICE_UUID)
            .ok_or("Command service not found")?;

        // Get characteristics from sensor service
        let sensor_chars = sensor_service.characteristics().await?;
        let bvp = sensor_chars.iter()
            .find(|c| c.uuid() == BVP_CHAR_UUID)
            .ok_or("BVP characteristic not found")?
            .clone();
        let eda = sensor_chars.iter()
            .find(|c| c.uuid() == EDA_CHAR_UUID)
            .ok_or("EDA characteristic not found")?
            .clone();
        let temp = sensor_chars.iter()
            .find(|c| c.uuid() == TEMP_CHAR_UUID)
            .ok_or("TEMP characteristic not found")?
            .clone();
        let acc = sensor_chars.iter()
            .find(|c| c.uuid() == ACC_CHAR_UUID)
            .ok_or("ACC characteristic not found")?
            .clone();

        // Get command characteristic
        let cmd_chars = cmd_service.characteristics().await?;
        let cmd = cmd_chars.iter()
            .find(|c| c.uuid() == CMD_CHAR_UUID)
            .ok_or("CMD characteristic not found")?
            .clone();

        self.characteristics = Some(DeviceCharacteristics {
            bvp, eda, temp, acc, cmd
        });

        self.device = Some(device.clone());

        // Get device name (NOT async in bluest)
        let name = device.name()
            .unwrap_or_else(|_| "Empatica E4".to_string());

        Ok(name)
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        // In bluest, disconnection happens when Device is dropped
        // or via adapter.disconnect_device()
        if let (Some(adapter), Some(device)) = (&self.adapter, &self.device) {
            let _ = adapter.disconnect_device(device).await;
        }
        self.device = None;
        self.characteristics = None;
        Ok(())
    }

    pub fn get_characteristics(&self) -> Option<DeviceCharacteristics> {
        self.characteristics.clone()
    }

    pub async fn start_streaming(&self) -> Result<(), Box<dyn Error>> {
        let chars = self.characteristics.as_ref().ok_or("Characteristics not initialized")?;

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as u32;

        let mut command = vec![0x01];
        command.extend_from_slice(&timestamp.to_le_bytes());

        println!("[BLE] Command: {:?}", command);

        // WAIT A BIT - let notifications fully enable on device side
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

        chars.cmd.write_without_response(&command).await?;

        println!("[BLE] Command sent");
        Ok(())
    }
    
    pub async fn stop_streaming(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

impl Clone for DeviceCharacteristics {
    fn clone(&self) -> Self {
        Self {
            bvp: self.bvp.clone(),
            eda: self.eda.clone(),
            temp: self.temp.clone(),
            acc: self.acc.clone(),
            cmd: self.cmd.clone(),
        }
    }
}