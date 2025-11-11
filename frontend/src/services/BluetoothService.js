// services/BluetoothService.js
import DataParser from './DataParser.js';

class BluetoothService {
    constructor() {
        this.device = null;
        this.server = null;
        this.isConnected = false;

        // YOUR CUSTOM LIBRARY INSTANCE HERE
        // this.customDevice = new YourCustomLibrary();
    }

    async connect() {
        try {
            this.device = await navigator.bluetooth.requestDevice({
                filters: [
                    { name: 'YOUR_DEVICE_NAME' }
                ],
                optionalServices: [
                    'YOUR_SERVICE_UUID'
                ]
            });

            console.log('Device selected:', this.device.name);

            this.server = await this.device.gatt.connect();
            console.log('Connected to GATT server');

            // TODO: Initialize your custom library here
            // await this.customDevice.init(this.server);

            this.isConnected = true;
            this.device.addEventListener('gattserverdisconnected', this.onDisconnected.bind(this));

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
            // TODO: Setup your characteristic notifications
            // Example structure (adapt to your library):

            // BVP
            // await this.customDevice.startNotify('BVP_UUID', (data) => {
            //   DataParser.parseBVP(data);
            // });

            // Temperature
            // await this.customDevice.startNotify('TEMP_UUID', (data) => {
            //   DataParser.parseTemperature(data);
            // });

            // EDA
            // await this.customDevice.startNotify('EDA_UUID', (data) => {
            //   DataParser.parseEDA(data);
            // });

            // Accelerometer
            // await this.customDevice.startNotify('ACC_UUID', (data) => {
            //   DataParser.parseAccelerometer(data);
            // });

            console.log('Started reading from device');

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
            // TODO: Stop notifications
            // await this.customDevice.stopNotify();

            console.log('Stopped reading from device');

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
    }

    onDisconnected() {
        console.log('Device disconnected');
        this.isConnected = false;
    }
}

export default new BluetoothService();