// App.vue
<template>
  <div class="vh-100 page-bg p-3">
    <div class="d-flex gap-3 h-100">
      <Sidebar />
      <div class="flex-grow-1 d-flex" style="min-width: 0;">
        <router-view/>
      </div>
    </div>
  </div>
</template>

<script>
import Sidebar from './components/Sidebar.vue';

export default {
  components: {
    Sidebar
  },
  created() {
    // Force clear URL query params
    const cleanPath = window.location.pathname;
    if (window.location.search) {
      window.history.replaceState({}, '', cleanPath);
      this.$router.replace({ path: cleanPath });
    }
  }
}
</script>