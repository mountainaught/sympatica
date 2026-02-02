use btleplug::api::{
    Central, Manager as _, Peripheral as _, ScanFilter, WriteType, CharPropFlags,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use crate::ble::constants::*;

pub struct BleManager {
    adapter: Option<Adapter>,
    device: Option<Peripheral>,
}

impl BleManager {
    pub fn new() -> Self {
        Self {
            adapter: None,
            device: None,
        }
    }

    pub async fn scan_and_connect(&mut self) -> Result<String, Box<dyn Error>> {
        // Get bluetooth adapter
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        let adapter = adapters.into_iter().next()
            .ok_or("No Bluetooth adapter found")?;

        self.adapter = Some(adapter.clone());

        // Start scanning
        adapter.start_scan(ScanFilter::default()).await?;
        time::sleep(Duration::from_secs(5)).await;

        // Find E4 device
        let peripherals = adapter.peripherals().await?;

        for peripheral in peripherals {
            let props = peripheral.properties().await?;
            let local_name = props
                .and_then(|p| p.local_name)
                .unwrap_or_default();

            if local_name.contains(DEVICE_NAME) {
                println!("Found E4: {}", local_name);

                // Connect
                peripheral.connect().await?;
                peripheral.discover_services().await?;

                self.device = Some(peripheral);
                return Ok(local_name);
            }
        }

        Err("Empatica E4 not found".into())
    }

    pub fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    pub async fn disconnect(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(device) = &self.device {
            device.disconnect().await?;
        }
        self.device = None;
        Ok(())
    }
}