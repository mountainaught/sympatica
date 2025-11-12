// services/parsers/TempParser.js

const TEMP_CALIBRATION = 0.46;

class TempParser {
    /**
     * Parse Temperature packet from E4
     * 12 bytes: 4 samples of unsigned 16-bit (2-byte) little-endian + 4 bytes metadata
     * Formula: (raw * 0.02) - 276.0 + calibration
     * Converts from Kelvin to Celsius
     */
    parse(dataView) {
        if (dataView.byteLength < 12) {
            return null;
        }

        const tempReadings = [];
        let offset = 0;

        // Read 4 samples (2 bytes each = 8 bytes total)
        while (offset < 8) {
            // Little-endian unsigned 16-bit
            const raw = dataView.getUint16(offset, true);

            // Convert: Kelvin → Celsius + calibration
            const temp = ((raw * 0.02) - 276.0) + TEMP_CALIBRATION;

            tempReadings.push(Math.round(temp * 100) / 100); // Round to 2 decimals
            offset += 2;
        }

        return tempReadings.length > 0 ? tempReadings : null;
    }
}

export default new TempParser();