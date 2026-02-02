// src/utils/Toast.vue
<template>
  <div class="toast-container position-fixed bottom-0 end-0 p-3">
    <div
        v-for="toast in toasts"
        :key="toast.id"
        class="toast show"
        role="alert">
      <div class="toast-header" :class="getToastClass(toast.type)">
        <i class="bi me-2" :class="getToastIcon(toast.type)"></i>
        <strong class="me-auto">{{ toast.title }}</strong>
        <button type="button" class="btn-close btn-close-white" @click="removeToast(toast.id)"></button>
      </div>
      <div class="toast-body">
        {{ toast.message }}
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      toasts: []
    }
  },
  methods: {
    addToast({ title, message, type = 'info', duration = 4000 }) {
      const id = Date.now();
      this.toasts.push({ id, title, message, type });

      setTimeout(() => {
        this.removeToast(id);
      }, duration);
    },

    removeToast(id) {
      this.toasts = this.toasts.filter(t => t.id !== id);
    },

    getToastClass(type) {
      const classes = {
        success: 'bg-success text-white',
        error: 'bg-danger text-white',
        warning: 'bg-warning text-dark',
        info: 'bg-primary text-white'
      };
      return classes[type] || classes.info;
    },

    getToastIcon(type) {
      const icons = {
        success: 'bi-check-circle-fill',
        error: 'bi-x-circle-fill',
        warning: 'bi-exclamation-triangle-fill',
        info: 'bi-info-circle-fill'
      };
      return icons[type] || icons.info;
    }
  }
}
</script>