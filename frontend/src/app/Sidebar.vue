<template>
  <div class="bg-dark text-white p-3" style="width: 220px;">
    <h4 class="mb-4">Menu</h4>
    <div class="d-grid gap-2">
      <button class="btn btn-secondary text-start" :class="{ 'btn-primary': currentPage === 'home' }" @click="navigate('home')">
        🏠 Home
      </button>
      <button class="btn btn-secondary text-start" :class="{ 'btn-primary': currentPage === 'patients' }" @click="navigate('patients')">
        👥 Patients
      </button>
      <button class="btn btn-secondary text-start" :class="{ 'btn-primary': currentPage === 'sessions' }" @click="navigate('sessions')">
        📊 Sessions
      </button>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      currentPage: 'home'
    }
  },
  methods: {
    navigate(page) {
      this.currentPage = page;
      this.$emit('navigate', page);
    }
  }
}
</script>