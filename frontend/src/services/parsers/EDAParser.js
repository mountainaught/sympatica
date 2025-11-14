// services/parsers/EDAParser.js
class EDAParser {
    /**
     * Parse EDA/GSR packet from E4
     * 20 bytes: 6 samples of 24-bit (3-byte) big-endian values + 2 bytes counter
     * Conversion: 1000000 / raw_value = microsiemens (µS)
     */
    parse(dataView) {
        if (dataView.byteLength < 20) {
            return null;
        }

        const readings = [];
        let offset = 0;

        // Read 6 samples (3 bytes each = 18 bytes total)
        while (offset + 3 <= dataView.byteLength - 2) {
            const byte1 = dataView.getUint8(offset);
            const byte2 = dataView.getUint8(offset + 1);
            const byte3 = dataView.getUint8(offset + 2);

            // Combine into 24-bit value
            const rawValue = (byte1 << 16) | (byte2 << 8) | byte3;

            // Convert to microsiemens
            const edaMicrosiemens = rawValue > 0 ? 1000000.0 / rawValue : 0;
            readings.push(Math.round(edaMicrosiemens * 1000) / 1000); // 3 decimals
            offset += 3;
        }

        return readings.length > 0 ? readings : null;
    }
}

export default new EDAParser();