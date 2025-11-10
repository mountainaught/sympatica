<template>
  <div class="d-flex vh-100">
    <Sidebar />
    <div class="flex-grow-1 d-flex flex-column">
      <TopBar />
      <LiveStats />
    </div>
  </div>
</template>

<script>
import Sidebar from './app/Sidebar.vue';
import TopBar from './app/DeviceStatus.vue';
import LiveReads from './app/LiveReads.vue';

export default {
  components: { Sidebar, TopBar, LiveStats: LiveReads }
}
</script>