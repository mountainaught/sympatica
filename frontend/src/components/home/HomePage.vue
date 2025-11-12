<template>
  <div class="flex-grow-1 d-flex flex-column gap-3">
    <DeviceStatus
        :activeSession="activeSession"
        @device-connected="onDeviceConnected"
        @recording-started="onRecordingStarted"
        @recording-stopped="onRecordingStopped" />
    <LiveStats :activeSessionId="activeSession?.id" />
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
  props: {
    activeSession: Object
  },
  methods: {
    onDeviceConnected(deviceInfo) {
      console.log('Device connected:', deviceInfo);
    },

    onRecordingStarted() {
      if (!this.activeSession) {
        alert('Please select an active session in the Patients page first!');
        return;
      }
      console.log('Recording to session:', this.activeSession.id);
    },

    onRecordingStopped() {
      console.log('Recording stopped');
    }
  }
}
</script>