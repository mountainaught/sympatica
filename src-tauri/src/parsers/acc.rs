#[derive(Debug, Clone)]
pub struct AccReading {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct AccParser;

impl AccParser {
    pub fn parse(data: &[u8]) -> Vec<AccReading> {
        let mut readings = Vec::new();
        let mut offset = 0;

        // Read all 3-byte samples (X, Y, Z)
        while offset + 3 <= data.len() {
            let x = data[offset] as i8;
            let y = data[offset + 1] as i8;
            let z = data[offset + 2] as i8;

            readings.push(AccReading {
                x: ((x as f32 / 64.0) * 100.0).round() / 100.0,
                y: ((y as f32 / 64.0) * 100.0).round() / 100.0,
                z: ((z as f32 / 64.0) * 100.0).round() / 100.0,
            });

            offset += 3;
        }

        readings
    }
}