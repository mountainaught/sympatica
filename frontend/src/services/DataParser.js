// services/DataParser.js
class DataParser {
    constructor() {
        this.onDataCallback = null;
        this.currentSessionId = null;
    }

    setSession(sessionId) {
        this.currentSessionId = sessionId;
    }

    onData(callback) {
        this.onDataCallback = callback;
    }

    // BVP Parser
    parseBVP(data) {
        // TODO: Parse your BVP data format
        // Example:
        const value = data.getFloat32(0, true); // adjust based on your format

        const reading = {
            type: 'bvp',
            value: value,
            unit: '',
            timestamp: new Date().toISOString()
        };

        this.handleReading(reading);
    }

    // Temperature Parser
    parseTemperature(data) {
        // TODO: Parse your temperature data format
        const value = data.getFloat32(0, true);

        const reading = {
            type: 'temperature',
            value: value,
            unit: '°C',
            timestamp: new Date().toISOString()
        };

        this.handleReading(reading);
    }

    // EDA Parser
    parseEDA(data) {
        // TODO: Parse your EDA data format
        const value = data.getFloat32(0, true);

        const reading = {
            type: 'eda',
            value: value,
            unit: 'µS',
            timestamp: new Date().toISOString()
        };

        this.handleReading(reading);
    }

    // Accelerometer Parser
    parseAccelerometer(data) {
        // TODO: Parse your accelerometer data format
        // Usually 3-axis data
        const x = data.getFloat32(0, true);
        const y = data.getFloat32(4, true);
        const z = data.getFloat32(8, true);

        const reading = {
            type: 'acc',
            value: { x, y, z },
            unit: 'g',
            timestamp: new Date().toISOString()
        };

        this.handleReading(reading);
    }

    async handleReading(reading) {
        // 1. Update UI (send to LiveStats)
        if (this.onDataCallback) {
            this.onDataCallback(reading);
        }

        // 2. Save to database
        if (this.currentSessionId) {
            await this.saveReading(reading);
        }
    }

    async saveReading(reading) {
        try {
            // For accelerometer, save each axis separately
            if (reading.type === 'acc') {
                await this.saveAccReading(reading);
                return;
            }

            const response = await fetch('http://localhost:8000/api/readings/create/', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    session_id: this.currentSessionId,
                    reading_type: reading.type,
                    value: reading.value,
                    unit: reading.unit
                })
            });

            if (!response.ok) {
                console.error('Failed to save reading');
            }
        } catch (error) {
            console.error('Error saving reading:', error);
        }
    }

    async saveAccReading(reading) {
        // Save X, Y, Z as separate readings or combined
        // Option 1: Combined as JSON
        await fetch('http://localhost:8000/api/readings/create/', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                session_id: this.currentSessionId,
                reading_type: 'acc',
                value: JSON.stringify(reading.value), // { x, y, z }
                unit: reading.unit
            })
        });

        // Option 2: Separate readings (you could save magnitude instead)
        // const magnitude = Math.sqrt(reading.value.x**2 + reading.value.y**2 + reading.value.z**2);
        // ... save magnitude
    }
}

export default new DataParser();