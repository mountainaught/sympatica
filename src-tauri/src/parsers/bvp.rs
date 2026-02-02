const FIR_COEF: [f32; 7] = [0.05, 0.1, 0.2, 0.3, 0.2, 0.1, 0.05];
const BVP_SCALE_FACTOR: f32 = 10.0;

pub struct BvpParser {
    // Offset tracking
    green_offset: i32,
    red_offset: i32,

    // FIR filter buffers (circular buffers of size 7)
    fir1_buffer: [f32; 7],
    fir2_buffer: [f32; 7],
    fir3_buffer: [f32; 7],

    // Kalman filter state
    kalman_p: f32,  // Error covariance
    kalman_x: f32,  // Estimated state
    kalman_q: f32,  // Process noise
    kalman_r: f32,  // Measurement noise
}

impl BvpParser {
    pub fn new() -> Self {
        Self {
            green_offset: 0,
            red_offset: 0,
            fir1_buffer: [0.0; 7],
            fir2_buffer: [0.0; 7],
            fir3_buffer: [0.0; 7],
            kalman_p: 1.0,
            kalman_x: 0.0,
            kalman_q: 0.01,
            kalman_r: 0.1,
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Vec<f32> {
        if data.len() < 20 {
            return Vec::new();
        }

        // Stage 1: Decode 7-bit delta encoding
        let decoded = self.decode_7bit_delta(data);

        // Stage 2-7: Process through filter pipeline
        let mut bvp_readings = Vec::new();

        let samples = (decoded.len() / 2).min(11);
        for i in 0..samples {
            let green = decoded[i * 2] as f32;
            let red = decoded[i * 2 + 1] as f32;

            // Update offsets
            self.red_offset += green as i32;
            self.green_offset += red as i32;

            // Stage 3: FIR Filter 1 - red channel
            Self::shift_buffer(&mut self.fir1_buffer, red);
            let filtered_red = Self::apply_fir(&self.fir1_buffer);

            // Stage 4: FIR Filter 2 - weighted combination
            let weighted = (red + green * 10.0) / 11.0;
            Self::shift_buffer(&mut self.fir2_buffer, weighted);
            let filtered_weighted = Self::apply_fir(&self.fir2_buffer);

            // Stage 5: Kalman Filter - fuse both signals
            let kalman_out = self.kalman_filter(filtered_red, filtered_weighted);

            // Stage 6: FIR Filter 3 - final smoothing
            Self::shift_buffer(&mut self.fir3_buffer, kalman_out);
            let filtered_kalman = Self::apply_fir(&self.fir3_buffer);

            // Stage 7: Scale and output
            let bvp = -filtered_kalman * BVP_SCALE_FACTOR;
            bvp_readings.push((bvp * 1000.0).round() / 1000.0); // 3 decimals
        }

        bvp_readings
    }

    fn decode_7bit_delta(&self, data: &[u8]) -> Vec<i32> {
        let mut decoded = Vec::new();
        let mut u_var28: u32 = 0;

        // Process 20 bytes (0x14)
        for u_var37 in 0..0x14 {
            let b_var21 = data[u_var37] as u32;
            let i_var31 = u_var37 % 7;
            let u_var36 = i_var31 + 1;

            u_var28 = (b_var21 >> u_var36) | u_var28;

            let mut output_val = (u_var28 & 0x7F) as i32;
            if output_val & 0x40 != 0 {
                output_val |= 0x80;
            }
            if output_val > 127 {
                output_val -= 256;
            }
            decoded.push(output_val);

            let mask = (1 << u_var36) - 1;
            u_var28 = ((b_var21 & mask) << (6 - i_var31)) & 0xFF;

            if u_var36 == 7 {
                let mut output_val = (u_var28 & 0x7F) as i32;
                if output_val & 0x40 != 0 {
                    output_val |= 0x80;
                }
                if output_val > 127 {
                    output_val -= 256;
                }
                decoded.push(output_val);
                u_var28 = 0;
            }
        }

        // Final value
        let mut final_val = (data[0x13] & 0x3F) as i32;
        if final_val & 0x20 != 0 {
            final_val |= 0xC0;
        }
        if final_val > 127 {
            final_val -= 256;
        }
        decoded.push(final_val);

        decoded
    }

    fn shift_buffer(buffer: &mut [f32; 7], new_value: f32) {
        buffer.rotate_left(1);
        buffer[6] = new_value;
    }

    fn apply_fir(buffer: &[f32; 7]) -> f32 {
        let mut sum = 0.0;
        for i in 0..7 {
            sum += FIR_COEF[i] * buffer[i];
        }
        sum
    }

    fn kalman_filter(&mut self, measurement1: f32, measurement2: f32) -> f32 {
        // Average the two measurements
        let measurement = (measurement1 + measurement2) / 2.0;

        // Prediction step
        let p_pred = self.kalman_p + self.kalman_q;

        // Update step
        let kalman_gain = p_pred / (p_pred + self.kalman_r);
        self.kalman_x = self.kalman_x + kalman_gain * (measurement - self.kalman_x);
        self.kalman_p = (1.0 - kalman_gain) * p_pred;

        // Adaptive covariance
        let diff1 = (measurement1 - measurement2).abs();
        if diff1 > 20.0 {
            // Signals diverge - increase uncertainty
            self.kalman_p = (self.kalman_p * 1.2).min(10.0);
        } else {
            // Signals agree - decrease uncertainty
            self.kalman_p = (self.kalman_p * 0.95).max(0.01);
        }

        self.kalman_x
    }
}