// services/parsers/AccParser.js

class AccParser {
    /**
     * Parse Accelerometer packet from E4
     * 3-axis signed bytes (X, Y, Z)
     * Divide by 64 to get g-force
     */
    parse(dataView) {
        const accReadings = [];
        let offset = 0;

        // Read all 3-byte samples (X, Y, Z)
        while (offset + 3 <= dataView.byteLength) {
            // Signed 8-bit values
            const x = dataView.getInt8(offset);
            const y = dataView.getInt8(offset + 1);
            const z = dataView.getInt8(offset + 2);

            accReadings.push({
                x: Math.round((x / 64.0) * 100) / 100,
                y: Math.round((y / 64.0) * 100) / 100,
                z: Math.round((z / 64.0) * 100) / 100
            });

            offset += 3;
        }

        return accReadings.length > 0 ? accReadings : null;
    }
}

export default new AccParser();