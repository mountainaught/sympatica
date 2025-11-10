<template>
  <div class="bg-light border-bottom p-3">
    <div class="d-flex justify-content-between align-items-center">
      <div class="d-flex align-items-center gap-2">
        <span class="badge rounded-pill" :class="isConnected ? 'bg-success' : 'bg-danger'">
          {{ isConnected ? '●' : '●' }}
        </span>
        <span>{{ isConnected ? 'Device Connected' : 'No Device' }}</span>
        <span v-if="isConnected" class="text-muted">| Battery: {{ batteryLevel }}%</span>
      </div>

      <div class="d-flex gap-2">
        <button class="btn btn-primary" @click="connectDevice" :disabled="isConnected">
          Connect Device
        </button>
        <button class="btn btn-success" @click="startRecording" :disabled="!isConnected || isRecording">
          Start Recording
        </button>
        <button class="btn btn-danger" @click="stopRecording" :disabled="!isRecording">
          Stop Recording
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      isConnected: false,
      isRecording: false,
      batteryLevel: 85
    }
  },
  methods: {
    connectDevice() {
      this.isConnected = true;
    },
    startRecording() {
      this.isRecording = true;
    },
    stopRecording() {
      this.isRecording = false;
    }
  }
}
</script>