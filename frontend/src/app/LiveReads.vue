<template>
  <div class="flex-grow-1 bg-light p-4 overflow-auto">
    <h2 class="mb-4">Live Readings</h2>

    <div class="row g-3">
      <div class="col-md-6 col-lg-3">
        <div class="card text-center">
          <div class="card-body">
            <h5 class="card-title">BVP</h5>
            <p class="display-4 text-primary fw-bold">{{ readings.bvp || '--' }}</p>
            <small class="text-muted">Blood Volume Pulse</small>
          </div>
        </div>
      </div>

      <div class="col-md-6 col-lg-3">
        <div class="card text-center">
          <div class="card-body">
            <h5 class="card-title">Temperature</h5>
            <p class="display-4 text-primary fw-bold">{{ readings.temperature || '--' }}</p>
            <small class="text-muted">°C</small>
          </div>
        </div>
      </div>

      <div class="col-md-6 col-lg-3">
        <div class="card text-center">
          <div class="card-body">
            <h5 class="card-title">EDA</h5>
            <p class="display-4 text-primary fw-bold">{{ readings.eda || '--' }}</p>
            <small class="text-muted">µS</small>
          </div>
        </div>
      </div>

      <div class="col-md-6 col-lg-3">
        <div class="card text-center">
          <div class="card-body">
            <h5 class="card-title">Accelerometer</h5>
            <p class="fs-4 text-primary fw-bold">
              X: {{ readings.acc_x || '--' }}<br>
              Y: {{ readings.acc_y || '--' }}<br>
              Z: {{ readings.acc_z || '--' }}
            </p>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  data() {
    return {
      readings: {
        bvp: null,
        temperature: null,
        eda: null,
        acc_x: null,
        acc_y: null,
        acc_z: null
      }
    }
  },
  mounted() {
    this.simulateReadings();
  },
  methods: {
    simulateReadings() {
      setInterval(() => {
        this.readings.bvp = (Math.random() * 100).toFixed(2);
        this.readings.temperature = (36 + Math.random() * 2).toFixed(1);
        this.readings.eda = (Math.random() * 10).toFixed(2);
        this.readings.acc_x = (Math.random() * 2 - 1).toFixed(2);
        this.readings.acc_y = (Math.random() * 2 - 1).toFixed(2);
        this.readings.acc_z = (Math.random() * 2 - 1).toFixed(2);
      }, 1000);
    }
  }
}
</script>