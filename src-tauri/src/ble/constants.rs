// BLE UUIDs for Empatica E4
use uuid::Uuid;

pub const DEVICE_NAME: &str = "Empatica E4";

// Services
pub const CMD_SERVICE_UUID: Uuid = Uuid::from_u128(0x00003e70_0000_1000_8000_00805f9b34fb);
pub const SENSOR_SERVICE_UUID: Uuid = Uuid::from_u128(0x00003ea0_0000_1000_8000_00805f9b34fb);

// Characteristics
pub const BVP_CHAR_UUID: Uuid = Uuid::from_u128(0x00003ea1_0000_1000_8000_00805f9b34fb);
pub const EDA_CHAR_UUID: Uuid = Uuid::from_u128(0x00003ea8_0000_1000_8000_00805f9b34fb);
pub const ACC_CHAR_UUID: Uuid = Uuid::from_u128(0x00003ea3_0000_1000_8000_00805f9b34fb);
pub const TEMP_CHAR_UUID: Uuid = Uuid::from_u128(0x00003ea6_0000_1000_8000_00805f9b34fb);
pub const CMD_CHAR_UUID: Uuid = Uuid::from_u128(0x00003e71_0000_1000_8000_00805f9b34fb);