// components/utils/Loading.vue
<template>
  <div v-if="show" class="loading-overlay" :class="{ 'overlay-fullscreen': fullscreen }">
    <div class="spinner-container">
      <div class="spinner-border text-primary" role="status" style="width: 3rem; height: 3rem;">
        <span class="visually-hidden">Loading...</span>
      </div>
      <p v-if="message" class="mt-3 text-muted fw-semibold">{{ message }}</p>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    show: {
      type: Boolean,
      default: false
    },
    message: {
      type: String,
      default: ''
    },
    fullscreen: {
      type: Boolean,
      default: false
    }
  }
}
</script>