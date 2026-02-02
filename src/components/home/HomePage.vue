// components/home/HomePage.vue
<template>
  <div class="flex-grow-1 d-flex flex-column gap-3">
    <DeviceStatus
        @device-connected="onDeviceConnected"
        @recording-started="onRecordingStarted"
        @recording-stopped="onRecordingStopped"
        @device-disconnected="onDeviceDisconnected" />
    <LiveStats :isRecording="isRecording" />
  </div>
</template>

<script>
import DeviceStatus from './DeviceStatus.vue';
import LiveStats from './LiveStats.vue';

export default {
  components: {
    DeviceStatus,
    LiveStats
  },
  data() {
    return {
      isRecording: false
    }
  },
  methods: {
    onDeviceConnected(deviceInfo) {
      console.log('Device connected:', deviceInfo);
    },

    onRecordingStarted() {
      const sessionId = this.$route.query.session;
      if (!sessionId) {
        alert('Please select an active session in the Patients page first!');
        return;
      }
      this.isRecording = true;
      console.log('Recording to session:', sessionId);
    },

    onRecordingStopped() {
      this.isRecording = false;
      console.log('Recording stopped');
    },

    onDeviceDisconnected() {
      this.isRecording = false;
      console.log('Device disconnected');
    }
  }
}
</script>