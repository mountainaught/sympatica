// components/home/VitalsCards.vue
<template>
  <div class="row g-3 mb-4">
    <div class="col-lg-3 col-md-6">
      <div class="vitals-card bg-light border-0 rounded-3 p-3">
        <div class="d-flex justify-content-between align-items-start mb-2">
          <span class="text-muted text-uppercase small fw-semibold">Blood Volume Pulse</span>
          <i class="bi bi-heart-pulse text-danger fs-5"></i>
        </div>
        <div class="d-flex align-items-baseline gap-2">
          <span class="display-5 fw-bold text-dark">{{ readings.bvp || '--' }}</span>
          <span class="text-muted">units</span>
        </div>
        <div class="mt-2">
          <span class="badge badge-status" :class="getStatusClass('bvp')">{{ getStatus('bvp') }}</span>
        </div>
      </div>
    </div>

    <div class="col-lg-3 col-md-6">
      <div class="vitals-card bg-light border-0 rounded-3 p-3">
        <div class="d-flex justify-content-between align-items-start mb-2">
          <span class="text-muted text-uppercase small fw-semibold">Skin Conductance</span>
          <i class="bi bi-droplet text-primary fs-5"></i>
        </div>
        <div class="d-flex align-items-baseline gap-2">
          <span class="display-5 fw-bold text-dark">{{ readings.eda || '--' }}</span>
          <span class="text-muted">µS</span>
        </div>
        <div class="mt-2">
          <span class="badge badge-status" :class="getStatusClass('eda')">{{ getStatus('eda') }}</span>
        </div>
      </div>
    </div>

    <div class="col-lg-3 col-md-6">
      <div class="vitals-card bg-light border-0 rounded-3 p-3">
        <div class="d-flex justify-content-between align-items-start mb-2">
          <span class="text-muted text-uppercase small fw-semibold">Skin Temperature</span>
          <i class="bi bi-thermometer-half text-warning fs-5"></i>
        </div>
        <div class="d-flex align-items-baseline gap-2">
          <span class="display-5 fw-bold text-dark">{{ readings.temperature || '--' }}</span>
          <span class="text-muted">°C</span>
        </div>
        <div class="mt-2">
          <span class="badge badge-status" :class="getStatusClass('temperature')">{{ getStatus('temperature') }}</span>
        </div>
      </div>
    </div>

    <div class="col-lg-3 col-md-6">
      <div class="vitals-card bg-light border-0 rounded-3 p-3">
        <div class="d-flex justify-content-between align-items-start mb-2">
          <span class="text-muted text-uppercase small fw-semibold">Accelerometer</span>
          <i class="bi bi-shuffle text-success fs-5"></i>
        </div>
        <div class="d-flex flex-column" style="font-size: 1.1rem;">
          <div class="d-flex justify-content-between">
            <span class="text-muted">X:</span>
            <span class="fw-bold">{{ readings.acc_x || '--' }}</span>
          </div>
          <div class="d-flex justify-content-between">
            <span class="text-muted">Y:</span>
            <span class="fw-bold">{{ readings.acc_y || '--' }}</span>
          </div>
          <div class="d-flex justify-content-between">
            <span class="text-muted">Z:</span>
            <span class="fw-bold">{{ readings.acc_z || '--' }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    readings: {
      type: Object,
      required: true
    }
  },
  methods: {
    getStatus(type) {
      if (!this.readings[type]) return 'NO DATA';
      return 'NORMAL';
    },

    getStatusClass(type) {
      if (!this.readings[type]) return 'bg-secondary';
      return 'bg-success';
    }
  }
}
</script>