// services/DataParser.js (BATCHED - using /readings/create/)
import BVPParser from './parsers/BVPParser.js';
import EDAParser from './parsers/EDAParser.js';
import TempParser from './parsers/TempParser.js';
import AccParser from './parsers/AccParser.js';
import { postAPI } from '../utils/helpers.js';

class DataParser {
    constructor() {
        this.onDataCallback = null;
        this.currentSessionId = null;

        // BATCH BUFFER
        this.readingBuffer = [];
        this.batchInterval = null;
        this.BATCH_SIZE = 50; // Send every 50 readings
        this.BATCH_TIME = 1000; // Or every 1 second
    }

    setSession(sessionId) {
        this.currentSessionId = sessionId;

        // Start batching when session is active
        if (sessionId && !this.batchInterval) {
            this.batchInterval = setInterval(() => this.flushBuffer(), this.BATCH_TIME);
        } else if (!sessionId && this.batchInterval) {
            clearInterval(this.batchInterval);
            this.batchInterval = null;
            this.flushBuffer().then(); // Send remaining readings
        }

        console.log('DataParser session set to:', sessionId);
    }

    onData(callback) {
        this.onDataCallback = callback;
    }

    // ========== PARSER ROUTING ==========

// services/DataParser.js (UPDATE)

    parseBVP(dataView) {
        const readings = BVPParser.parse(dataView);
        if (readings) {
            const baseTime = Date.now();
            const sampleInterval = 1000 / 64; // 64 Hz = ~15.6ms between samples

            readings.forEach((value, index) => {
                this.handleReading({
                    type: 'bvp',
                    value: value,
                    timestamp: new Date(baseTime + (index * sampleInterval)).toISOString()
                });
            });
        }
    }

    parseEDA(dataView) {
        const readings = EDAParser.parse(dataView);
        if (readings) {
            const baseTime = Date.now();
            const sampleInterval = 1000 / 4; // 4 Hz = 250ms between samples

            readings.forEach((value, index) => {
                this.handleReading({
                    type: 'eda',
                    value: value,
                    timestamp: new Date(baseTime + (index * sampleInterval)).toISOString()
                });
            });
        }
    }

    parseTemperature(dataView) {
        const readings = TempParser.parse(dataView);
        if (readings) {
            const baseTime = Date.now();
            const sampleInterval = 1000 / 4; // 4 Hz = 250ms between samples

            readings.forEach((value, index) => {
                this.handleReading({
                    type: 'temperature',
                    value: value,
                    timestamp: new Date(baseTime + (index * sampleInterval)).toISOString()
                });
            });
        }
    }

    parseAccelerometer(dataView) {
        const readings = AccParser.parse(dataView);
        if (readings) {
            const baseTime = Date.now();
            const sampleInterval = 1000 / 32; // 32 Hz = ~31.25ms between samples

            readings.forEach(({ x, y, z }, index) => {
                this.handleReading({
                    type: 'acc',
                    value: { x, y, z },
                    timestamp: new Date(baseTime + (index * sampleInterval)).toISOString()
                });
            });
        }
    }

    // ========== READING HANDLER ==========

    handleReading(reading) {
        // 1. Update UI (send to LiveStats)
        if (this.onDataCallback) {
            this.onDataCallback(reading);
        }

        // 2. Buffer for database (if session active)
        if (this.currentSessionId) {
            this.bufferReading(reading);
        }
    }

    bufferReading(reading) {
        console.log('Buffering:', reading.type, 'with timestamp:', reading.timestamp);
        if (reading.type === 'acc') {
            this.readingBuffer.push({
                session_id: this.currentSessionId,
                reading_type: reading.type,
                value: JSON.stringify(reading.value),
                timestamp: reading.timestamp  // ADD THIS!
            });
        } else {
            this.readingBuffer.push({
                session_id: this.currentSessionId,
                reading_type: reading.type,
                value: reading.value,
                timestamp: reading.timestamp  // ADD THIS!
            });
        }

        // Flush if buffer is full
        if (this.readingBuffer.length >= this.BATCH_SIZE) {
            this.flushBuffer().then();
        }
    }

    async flushBuffer() {
        if (this.readingBuffer.length === 0) return;

        const batch = [...this.readingBuffer];
        this.readingBuffer = [];

        try {
            // Send batch to /readings/create/
            await postAPI('/readings/create/', { readings: batch });
            // console.log(`Saved ${batch.length} readings`);
        } catch (error) {
            console.error('Error saving batch:', error);
        }
    }
}

export default new DataParser();