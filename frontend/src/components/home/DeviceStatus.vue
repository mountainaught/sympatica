<!-- TopBar.vue -->
<template>
  <div class="card shadow-lg border-0" style="border-radius: 20px;">
    <div class="card-body p-3">
      <div class="d-flex justify-content-between align-items-center">
        <div class="d-flex align-items-center gap-3">
          <span class="badge rounded-pill fs-6 px-3 py-2" :class="isConnected ? 'bg-success' : 'bg-secondary'">
            <i class="bi bi-circle-fill me-2"></i>
            {{ isConnected ? 'Device Connected' : 'No Device' }}
          </span>
          <span v-if="isConnected" class="badge bg-light text-dark rounded-pill px-3 py-2">
            🔋 {{ batteryLevel }}%
          </span>
        </div>

        <div class="d-flex gap-2">
          <button class="btn btn-primary rounded-pill px-4 shadow-sm" @click="connectDevice" :disabled="isConnected">
            Connect
          </button>
          <button class="btn btn-success rounded-pill px-4 shadow-sm" @click="startRecording" :disabled="!isConnected || isRecording">
            Start
          </button>
          <button class="btn btn-danger rounded-pill px-4 shadow-sm" @click="stopRecording" :disabled="!isRecording">
            Stop
          </button>
        </div>
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