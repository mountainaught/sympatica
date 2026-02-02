const TEMP_CALIBRATION: f32 = 0.46;

pub struct TempParser;

impl TempParser {
    pub fn parse(data: &[u8]) -> Vec<f32> {
        let mut readings = Vec::new();
        let mut offset = 0;

        // Read 4 samples (2 bytes each = 8 bytes total)
        while offset + 2 <= 8.min(data.len()) {
            // Little-endian unsigned 16-bit
            let raw = u16::from_le_bytes([data[offset], data[offset + 1]]);

            // Convert: Kelvin → Celsius + calibration
            let temp = ((raw as f32 * 0.02) - 276.0) + TEMP_CALIBRATION;
            readings.push((temp * 1000.0).round() / 1000.0); // 3 decimals

            offset += 2;
        }

        readings
    }
}