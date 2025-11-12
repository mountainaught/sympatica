// services/DataParser.js
import BVPParser from './parsers/BVPParser.js';
import EDAParser from './parsers/EDAParser.js';
import TempParser from './parsers/TempParser.js';
import AccParser from './parsers/AccParser.js';
import { postAPI } from '../utils/helpers.js';

class DataParser {
    constructor() {
        this.onDataCallback = null;
        this.currentSessionId = null;
    }

    setSession(sessionId) {
        this.currentSessionId = sessionId;
        console.log('DataParser session set to:', sessionId);
    }

    onData(callback) {
        this.onDataCallback = callback;
    }

    // ========== PARSER ROUTING ==========

    parseBVP(dataView) {
        const readings = BVPParser.parse(dataView);
        if (readings) {
            readings.forEach(value => {
                this.handleReading({
                    type: 'bvp',
                    value: value,
                    unit: '',
                    timestamp: new Date().toISOString()
                });
            });
        }
    }

    parseEDA(dataView) {
        const readings = EDAParser.parse(dataView);
        if (readings) {
            readings.forEach(value => {
                this.handleReading({
                    type: 'eda',
                    value: value,
                    unit: 'µS',
                    timestamp: new Date().toISOString()
                });
            });
        }
    }

    parseTemperature(dataView) {
        const readings = TempParser.parse(dataView);
        if (readings) {
            readings.forEach(value => {
                this.handleReading({
                    type: 'temperature',
                    value: value,
                    unit: '°C',
                    timestamp: new Date().toISOString()
                });
            });
        }
    }

    parseAccelerometer(dataView) {
        const readings = AccParser.parse(dataView);
        if (readings) {
            readings.forEach(({ x, y, z }) => {
                this.handleReading({
                    type: 'acc',
                    value: { x, y, z },
                    unit: 'g',
                    timestamp: new Date().toISOString()
                });
            });
        }
    }

    // ========== READING HANDLER ==========

    async handleReading(reading) {
        // 1. Update UI (send to LiveStats)
        if (this.onDataCallback) {
            this.onDataCallback(reading);
        }

        // 2. Save to database (if session active)
        if (this.currentSessionId) {
            await this.saveReading(reading);
        }
    }

    async saveReading(reading) {
        try {
            // For accelerometer, save as JSON string
            if (reading.type === 'acc') {
                await postAPI('/readings/create/', {
                    session_id: this.currentSessionId,
                    reading_type: reading.type,
                    value: JSON.stringify(reading.value), // {x, y, z} as string
                    unit: reading.unit
                });
                return;
            }

            // For other sensors, save value directly
            await postAPI('/readings/create/', {
                session_id: this.currentSessionId,
                reading_type: reading.type,
                value: reading.value,
                unit: reading.unit
            });

        } catch (error) {
            console.error('Error saving reading:', error);
        }
    }
}

export default new DataParser();