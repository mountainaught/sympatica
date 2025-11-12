// services/BluetoothService.js
import DataParser from './DataParser.js';

// E4 BLE Characteristic UUIDs
const DEVICE_NAME = 'Empatica E4';
const CMD_SERVICE_UUID = '00003e70-0000-1000-8000-00805f9b34fb';
const SENSOR_SERVICE_UUID = '00003ea0-0000-1000-8000-00805f9b34fb';

const BVP_CHARACTERISTIC_UUID = '00003ea1-0000-1000-8000-00805f9b34fb';
const EDA_CHARACTERISTIC_UUID = '00003ea8-0000-1000-8000-00805f9b34fb';
const ACC_CHARACTERISTIC_UUID = '00003ea3-0000-1000-8000-00805f9b34fb';
const TEMP_CHARACTERISTIC_UUID = '00003ea6-0000-1000-8000-00805f9b34fb';
const CMD_CHARACTERISTIC_UUID = '00003e71-0000-1000-8000-00805f9b34fb';

class BluetoothService {
    constructor() {
        this.device = null;
        this.server = null;
        this.isConnected = false;
        this.characteristics = {};
    }

    async connect() {
        try {
            console.log('Scanning for Empatica E4...');

            this.device = await navigator.bluetooth.requestDevice({
                filters: [{ name: DEVICE_NAME }],
                optionalServices: [CMD_SERVICE_UUID, SENSOR_SERVICE_UUID] // E4's main service
            });

            console.log('Device selected:', this.device.name);

            this.server = await this.device.gatt.connect();
            console.log('Connected to GATT server');

            const cmdService = await this.server.getPrimaryService(CMD_SERVICE_UUID);
            // Get the main E4 service
            const sensorService = await this.server.getPrimaryService(SENSOR_SERVICE_UUID);

            // Get all characteristics from that service
            this.characteristics.bvp = await sensorService.getCharacteristic(BVP_CHARACTERISTIC_UUID);
            this.characteristics.eda = await sensorService.getCharacteristic(EDA_CHARACTERISTIC_UUID);
            this.characteristics.acc = await sensorService.getCharacteristic(ACC_CHARACTERISTIC_UUID);
            this.characteristics.temp = await sensorService.getCharacteristic(TEMP_CHARACTERISTIC_UUID);
            this.characteristics.cmd = await cmdService.getCharacteristic(CMD_CHARACTERISTIC_UUID);

            this.isConnected = true;
            this.device.addEventListener('gattserverdisconnected', this.onDisconnected.bind(this));

            console.log('All characteristics obtained!');

            return {
                success: true,
                deviceName: this.device.name,
                deviceId: this.device.id
            };

        } catch (error) {
            console.error('Bluetooth connection error:', error);
            return {
                success: false,
                error: error.message
            };
        }
    }
    
    async startReading() {
        if (!this.isConnected) {
            throw new Error('Device not connected');
        }

        try {
            console.log('Setting up notifications...');

            // Setup notifications for each sensor
            await this.characteristics.bvp.startNotifications();
            this.characteristics.bvp.addEventListener('characteristicvaluechanged', (event) => {
                const data = event.target.value;
                DataParser.parseBVP(data);
            });

            await this.characteristics.eda.startNotifications();
            this.characteristics.eda.addEventListener('characteristicvaluechanged', (event) => {
                const data = event.target.value;
                DataParser.parseEDA(data);
            });

            await this.characteristics.temp.startNotifications();
            this.characteristics.temp.addEventListener('characteristicvaluechanged', (event) => {
                const data = event.target.value;
                DataParser.parseTemperature(data);
            });

            await this.characteristics.acc.startNotifications();
            this.characteristics.acc.addEventListener('characteristicvaluechanged', (event) => {
                const data = event.target.value;
                DataParser.parseAccelerometer(data);
            });

            console.log('Notifications setup complete!');

            // Send start command: 1 byte (0x01) + 4 bytes (unix timestamp)
            const timestamp = Math.floor(Date.now() / 1000);
            const command = new ArrayBuffer(5);
            const view = new DataView(command);
            view.setUint8(0, 1); // Command byte
            view.setUint32(1, timestamp, true); // Little-endian timestamp

            await this.characteristics.cmd.writeValue(command);
            console.log('Start command sent! E4 is streaming~');

        } catch (error) {
            console.error('Error starting readings:', error);
            throw error;
        }
    }

    async stopReading() {
        if (!this.isConnected) {
            return;
        }

        try {
            // Stop all notifications
            await this.characteristics.bvp.stopNotifications();
            await this.characteristics.eda.stopNotifications();
            await this.characteristics.temp.stopNotifications();
            await this.characteristics.acc.stopNotifications();

            console.log('Stopped reading from E4');

        } catch (error) {
            console.error('Error stopping readings:', error);
        }
    }

    async disconnect() {
        if (this.device && this.device.gatt.connected) {
            await this.stopReading();
            this.device.gatt.disconnect();
        }

        this.isConnected = false;
        this.device = null;
        this.server = null;
        this.characteristics = {};
    }

    onDisconnected() {
        console.log('E4 disconnected');
        this.isConnected = false;
    }
}

export default new BluetoothService();