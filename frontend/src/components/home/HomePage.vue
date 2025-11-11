<template>
  <div class="vh-100 bg-gradient p-3" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
    <div class="d-flex gap-3 h-100">
      <Sidebar :currentPage="'home'" @navigate="$emit('navigate', $event)" />
      <div class="flex-grow-1 d-flex flex-column gap-3">
        <TopBar />
        <LiveStats />
      </div>
    </div>
  </div>
</template>

<script>
import Sidebar from '../shared/Sidebar.vue';
import TopBar from './DeviceStatus.vue';
import LiveStats from './LiveReads.vue';

export default {
  components: { Sidebar, TopBar, LiveStats }
}
</script>