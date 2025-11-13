<template>
  <div class="card shadow-lg border-0 rounded-card">
    <div class="card-body p-3">
      <div class="d-flex justify-content-between align-items-center">
        <div class="d-flex align-items-center gap-3">
          <span class="badge rounded-pill fs-6 badge-status" :class="isConnected ? 'bg-success' : 'bg-secondary'">
            <i class="bi bi-circle-fill me-2"></i>
            {{ isConnected ? 'Device Connected' : 'No Device' }}
          </span>
          <span v-if="isConnected" class="badge rounded-pill fs-6 bg-light text-dark badge-device">
            {{ deviceName }}
          </span>
          <span v-if="sessionData" class="badge rounded-pill fs-6 bg-info text-dark badge-session">
            {{ sessionData.patient_name }} - {{ sessionData.session_name || 'Unnamed' }}
          </span>
          <span v-else class="badge rounded-pill fs-6 bg-warning text-dark badge-session">
            No Active Session
          </span>
        </div>

        <div class="d-flex gap-2">
          <button class="btn btn-primary rounded-pill px-4 shadow-sm btn-device-action" @click="connectDevice" :disabled="isConnected">
            Connect
          </button>
          <button class="btn btn-success rounded-pill px-4 shadow-sm btn-device-action" @click="startRecording" :disabled="!isConnected || isRecording || !sessionId">
            Start
          </button>
          <button class="btn btn-danger rounded-pill px-4 shadow-sm btn-device-action" @click="stopRecording" :disabled="!isRecording">
            Stop
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
      sessionData: null  // Store full session details
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
    // Load session on mount if in URL
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
      const result = await BluetoothService.connect();

      if (result.success) {
        this.isConnected = true;
        this.deviceName = result.deviceName;
        this.$emit('device-connected', result);
      } else {
        alert('Failed to connect: ' + result.error);
      }
    },

    async startRecording() {
      if (!this.sessionId) {
        alert('Please select an active session first!');
        return;
      }

      try {
        await BluetoothService.startReading();
        this.isRecording = true;
        this.$emit('recording-started');
      } catch (error) {
        alert('Failed to start recording: ' + error.message);
      }
    },

    async stopRecording() {
      await BluetoothService.stopReading();
      this.isRecording = false;
      this.$emit('recording-stopped');
    }
  },
  beforeUnmount() {
    BluetoothService.disconnect();
  }
}
</script>