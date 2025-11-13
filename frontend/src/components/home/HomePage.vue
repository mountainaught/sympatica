<template>
  <div class="flex-grow-1 d-flex flex-column gap-3">
    <DeviceStatus
        @device-connected="onDeviceConnected"
        @recording-started="onRecordingStarted"
        @recording-stopped="onRecordingStopped" />
    <LiveStats />
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
      console.log('Recording to session:', sessionId);
    },

    onRecordingStopped() {
      console.log('Recording stopped');
    }
  }
}
</script>