// FIR filter coefficients (from decompiled E4 code)
const FIR_COEF = [0.05, 0.1, 0.2, 0.3, 0.2, 0.1, 0.05];
const BVP_SCALE_FACTOR = 10.0;

class BVPParser {
    constructor() {
        // Offset tracking
        this.greenOffset = 0;
        this.redOffset = 0;

        // FIR filter buffers (circular buffers of size 7)
        this.fir1Buffer = new Array(FIR_COEF.length).fill(0.0);
        this.fir2Buffer = new Array(FIR_COEF.length).fill(0.0);
        this.fir3Buffer = new Array(FIR_COEF.length).fill(0.0);

        // Kalman filter state
        this.kalmanP = 1.0;  // Error covariance
        this.kalmanX = 0.0;  // Estimated state
        this.kalmanQ = 0.01; // Process noise
        this.kalmanR = 0.1;  // Measurement noise
    }

    /**
     * Parse BVP packet from E4
     * Flow: Decode (7-bit delta) → Parse → FIR1 → FIR2 → Kalman → FIR3 → Output
     * This is Empatica's $1500 "proprietary algorithm" uwu
     */
    parse(dataView) {
        if (dataView.byteLength < 20) {
            return null;
        }

        // ===== Stage 1: Decode 7-bit packed delta encoding =====
        const decoded = this.decode7BitDelta(dataView);

        // ===== Stage 2-7: Process through filter pipeline =====
        const bvpReadings = [];

        for (let i = 0; i < Math.min(11, Math.floor(decoded.length / 2)); i++) {
            const green = decoded[i * 2];
            const red = decoded[i * 2 + 1];

            // Update offsets
            this.redOffset += green;
            this.greenOffset += red;

            // === Stage 3: FIR Filter 1 - red channel ===
            this.fir1Buffer.push(red);
            this.fir1Buffer.shift(); // Keep buffer size at 7
            const filteredRed = this.applyFIR(this.fir1Buffer);

            // === Stage 4: FIR Filter 2 - weighted combination ===
            const weighted = (red + green * 10.0) / 11.0;
            this.fir2Buffer.push(weighted);
            this.fir2Buffer.shift();
            const filteredWeighted = this.applyFIR(this.fir2Buffer);

            // === Stage 5: Kalman Filter - fuse both signals ===
            const kalmanOut = this.kalmanFilter(filteredRed, filteredWeighted);

            // === Stage 6: FIR Filter 3 - final smoothing ===
            this.fir3Buffer.push(kalmanOut);
            this.fir3Buffer.shift();
            const filteredKalman = this.applyFIR(this.fir3Buffer);

            // === Stage 7: Scale and output ===
            const bvp = Math.round(-filteredKalman * BVP_SCALE_FACTOR * 100) / 100;
            bvpReadings.push(bvp);
        }

        return bvpReadings.length > 0 ? bvpReadings : null;
    }

    /**
     * Decode 7-bit packed delta encoding
     * This is the crazy loop from the decompiled Java code
     */
    decode7BitDelta(dataView) {
        const decoded = [];
        let uVar28 = 0;

        // Process 20 bytes (0x14)
        for (let uVar37 = 0; uVar37 < 0x14; uVar37++) {
            const bVar21 = dataView.getUint8(uVar37);
            const iVar31 = uVar37 % 7;
            const uVar36 = iVar31 + 1;

            uVar28 = (bVar21 >> uVar36) | uVar28;

            let outputVal = uVar28 & 0x7F;
            if (outputVal & 0x40) {
                outputVal |= 0x80;
            }
            if (outputVal > 127) {
                outputVal -= 256;
            }
            decoded.push(outputVal);

            const mask = (1 << uVar36) - 1;
            uVar28 = ((bVar21 & mask) << (6 - iVar31)) & 0xFF;

            if (uVar36 === 7) {
                let outputVal = uVar28 & 0x7F;
                if (outputVal & 0x40) {
                    outputVal |= 0x80;
                }
                if (outputVal > 127) {
                    outputVal -= 256;
                }
                decoded.push(outputVal);
                uVar28 = 0;
            }
        }

        // Final value
        let finalVal = dataView.getUint8(0x13) & 0x3F;
        if (finalVal & 0x20) {
            finalVal |= 0xC0;
        }
        if (finalVal > 127) {
            finalVal -= 256;
        }
        decoded.push(finalVal);

        return decoded;
    }

    /**
     * Apply FIR filter to buffer
     */
    applyFIR(buffer) {
        let sum = 0;
        for (let i = 0; i < FIR_COEF.length; i++) {
            sum += FIR_COEF[i] * buffer[i];
        }
        return sum;
    }

    /**
     * Kalman filter to fuse two measurements
     * Based on decompiled KalmanFilter_evaluate()
     */
    kalmanFilter(measurement1, measurement2) {
        // Average the two measurements
        const measurement = (measurement1 + measurement2) / 2.0;

        // Prediction step
        const pPred = this.kalmanP + this.kalmanQ;

        // Update step
        const kalmanGain = pPred / (pPred + this.kalmanR);
        this.kalmanX = this.kalmanX + kalmanGain * (measurement - this.kalmanX);
        this.kalmanP = (1 - kalmanGain) * pPred;

        // Adaptive covariance (simplified)
        const diff1 = Math.abs(measurement1 - measurement2);
        if (diff1 > 20) {
            // Signals diverge - increase uncertainty
            this.kalmanP = Math.min(this.kalmanP * 1.2, 10.0);
        } else {
            // Signals agree - decrease uncertainty
            this.kalmanP = Math.max(this.kalmanP * 0.95, 0.01);
        }

        return this.kalmanX;
    }
}

export default new BVPParser();