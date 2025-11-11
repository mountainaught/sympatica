<template>
  <div class="vh-100 bg-gradient p-3" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
    <div class="d-flex gap-3 h-100">
      <Sidebar :currentPage="'home'" @navigate="$emit('navigate', $event)" />
      <div class="flex-grow-1 d-flex flex-column gap-3">
        <TopBar
            :activeSession="activeSession"
            @device-connected="onDeviceConnected"
            @recording-started="onRecordingStarted"
            @recording-stopped="onRecordingStopped" />
        <LiveStats :activeSessionId="activeSession?.id" />
      </div>
    </div>
  </div>
</template>

<script>
import Sidebar from '../shared/Sidebar.vue';
import TopBar from './DeviceStatus.vue';
import LiveStats from './LiveStats.vue';

export default {
  components: { Sidebar, TopBar, LiveStats },
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