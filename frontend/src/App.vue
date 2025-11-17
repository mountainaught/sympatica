// App.vue
<template>
  <div class="vh-100 page-bg p-3">
    <div class="d-flex gap-3 h-100">
      <Sidebar />
      <div class="flex-grow-1 d-flex" style="min-width: 0;">
        <transition name="fade" mode="out-in">
          <router-view :key="$route.path" />
        </transition>
      </div>
    </div>

    <!-- Toast Container -->
    <Toast ref="toast" />
  </div>
</template>

<script>
import Sidebar from './components/Sidebar.vue';
import Toast from './utils/Toast.vue';

export default {
  components: {
    Sidebar,
    Toast
  },
  created() {
    const cleanPath = window.location.pathname;
    if (window.location.search) {
      window.history.replaceState({}, '', cleanPath);
      this.$router.replace({ path: cleanPath });
    }
  },
  mounted() {
    // Make toast globally accessible
    window.$toast = this.$refs.toast;
  }
}
</script>