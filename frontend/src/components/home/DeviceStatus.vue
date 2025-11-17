// components/home/DeviceStatus.vue
<template>
  <div class="card shadow-lg border-0 rounded-card">
    <div class="card-body p-3">
      <div class="d-flex justify-content-between align-items-center">
        <div class="d-flex align-items-center gap-3">
          <span class="badge rounded-pill fs-6 badge-status" :class="isConnected ? 'bg-success' : 'bg-secondary'">
            <i class="bi" :class="isConnected ? 'bi-bluetooth' : 'bi-bluetooth-slash'"></i>
            <span class="ms-2">{{ isConnected ? 'Device Connected' : 'No Device' }}</span>
          </span>

                  <span v-if="isConnected" class="badge rounded-pill fs-6 bg-light text-dark badge-device">
            <i class="bi bi-cpu me-2"></i>
            {{ deviceName }}
          </span>

                  <span v-if="sessionData" class="badge rounded-pill fs-6 bg-info text-dark badge-session">
            <i class="bi bi-clipboard-data me-2"></i>
            {{ sessionData.patient_name }} - {{ sessionData.session_name || 'Unnamed' }}
          </span>
                  <span v-else class="badge rounded-pill fs-6 bg-warning text-dark badge-session">
            <i class="bi bi-exclamation-triangle me-2"></i>
            No Active Session
          </span>
        </div>

        <div class="d-flex gap-2">
          <button class="btn btn-primary rounded-pill px-4 shadow-sm btn-device-action" @click="connectDevice" :disabled="isConnected || connecting">
            <span v-if="connecting" class="spinner-border spinner-border-sm me-2" role="status"></span>
            {{ connecting ? 'Connecting...' : 'Connect' }}
          </button>
          <button class="btn btn-success rounded-pill px-4 shadow-sm btn-device-action" @click="startRecording" :disabled="!isConnected || isRecording || !sessionId">
            Start
          </button>
          <button class="btn btn-danger rounded-pill px-4 shadow-sm btn-device-action" @click="stopRecording" :disabled="!isRecording">
            Stop
          </button>
          <button class="btn btn-secondary rounded-pill px-4 shadow-sm btn-device-action" @click="disconnectDevice" :disabled="!isConnected">
            Disconnect
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import BluetoothService from '../../services/BluetoothService.js';
import { fetchAPI } from '../../utils/helpers.js';

export default {
  data() {
    return {
      isConnected: false,
      isRecording: false,
      deviceName: '',
      sessionData: null,
      connecting: false  // ADD THIS
    }
  },
  computed: {
    sessionId() {
      return this.$route.query.session || null;
    }
  },
  watch: {
    async sessionId(newSessionId) {
      if (newSessionId) {
        try {
          this.sessionData = await fetchAPI(`/sessions/${newSessionId}/`);
        } catch (error) {
          console.error('Error loading session:', error);
          this.sessionData = null;
        }
      } else {
        this.sessionData = null;
      }
    }
  },
  async mounted() {
    // Check if already connected
    const status = BluetoothService.getConnectionStatus();
    if (status.isConnected) {
      this.isConnected = true;
      console.log(status.deviceName, 'is connected');
      this.deviceName = status.deviceName;
    }

    // Load session if in URL
    if (this.sessionId) {
      try {
        this.sessionData = await fetchAPI(`/sessions/${this.sessionId}/`);
      } catch (error) {
        console.error('Error loading session:', error);
      }
    }
  },
  methods: {
    async connectDevice() {
      this.connecting = true;
      const result = await BluetoothService.connect();

      if (result.success) {
        this.isConnected = true;
        this.deviceName = result.deviceName;
        this.$emit('device-connected', result);
        window.$toast.addToast({
          title: 'Connected',
          message: `Successfully connected to ${result.deviceName}`,
          type: 'success'
        });
      } else {
        window.$toast.addToast({
          title: 'Connection Failed',
          message: result.error,
          type: 'error'
        });
      }
      this.connecting = false;
    },

    async startRecording() {
      if (!this.sessionId) {
        window.$toast.addToast({
          title: 'No Session',
          message: 'Please select an active session first!',
          type: 'warning'
        });
        return;
      }

      try {
        await BluetoothService.startReading();
        this.isRecording = true;
        this.$emit('recording-started');
        window.$toast.addToast({
          title: 'Recording Started',
          message: 'Now recording physiological data',
          type: 'success'
        });
      } catch (error) {
        window.$toast.addToast({
          title: 'Recording Failed',
          message: error.message,
          type: 'error'
        });
      }
    },

    async stopRecording() {
      await BluetoothService.stopReading();
      this.isRecording = false;
      this.$emit('recording-stopped');
      window.$toast.addToast({
        title: 'Recording Stopped',
        message: 'Data recording has been stopped',
        type: 'info'
      });
    },

    async disconnectDevice() {
      await BluetoothService.disconnect();
      this.isConnected = false;
      this.isRecording = false;
      this.deviceName = '';
      this.$emit('device-disconnected');
    }
  }
}
</script>