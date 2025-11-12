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
          <span v-if="activeSession" class="badge rounded-pill fs-6 bg-info text-dark badge-session">
            {{ activeSession.patient_name }} - {{ activeSession.session_name || 'Unnamed' }}
          </span>
          <span v-else class="badge rounded-pill fs-6 bg-warning text-dark badge-session">
            No Active Session
          </span>
        </div>

        <div class="d-flex gap-2">
          <button class="btn btn-primary rounded-pill px-4 shadow-sm btn-device-action" @click="connectDevice" :disabled="isConnected">
            Connect
          </button>
          <button class="btn btn-success rounded-pill px-4 shadow-sm btn-device-action" @click="startRecording" :disabled="!isConnected || isRecording || !activeSession">
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

export default {
  props: {
    activeSession: Object
  },
  data() {
    return {
      isConnected: false,
      isRecording: false,
      deviceName: ''
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
      if (!this.activeSession) {
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