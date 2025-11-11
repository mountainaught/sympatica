<template>
  <div class="card shadow-lg border-0" style="flex: 0 0 220px; border-radius: 20px;">
    <div class="card-body p-4">
      <h4 class="fw-bold mb-4 text-center">Menu</h4>
      <div class="d-grid gap-3">
        <button
            class="btn btn-lg rounded-pill shadow-sm"
            :class="currentPage === 'home' ? 'btn-primary' : 'btn-light'"
            @click="navigate('home')">
          🏠 Home
        </button>
        <button
            class="btn btn-lg rounded-pill shadow-sm"
            :class="currentPage === 'patients' ? 'btn-primary' : 'btn-light'"
            @click="navigate('patients')">
          👥 Patients
        </button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: ['currentPage'],
  methods: {
    navigate(page) {
      this.$emit('navigate', page);
    }
  }
}
</script>