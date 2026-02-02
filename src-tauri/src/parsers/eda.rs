pub struct EdaParser;

impl EdaParser {
    pub fn parse(data: &[u8]) -> Vec<f32> {
        let mut readings = Vec::new();
        let mut offset = 0;

        // Read 6 samples (3 bytes each = 18 bytes total)
        while offset + 3 <= data.len().saturating_sub(2) {
            let byte1 = data[offset] as u32;
            let byte2 = data[offset + 1] as u32;
            let byte3 = data[offset + 2] as u32;

            // Combine into 24-bit value (big-endian)
            let raw_value = (byte1 << 16) | (byte2 << 8) | byte3;

            // Convert to microsiemens
            let eda = if raw_value > 0 {
                1_000_000.0 / raw_value as f32
            } else {
                0.0
            };

            readings.push((eda * 1000.0).round() / 1000.0); // 3 decimals
            offset += 3;
        }

        readings
    }
}