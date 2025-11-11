<!-- LiveStats.vue -->
<template>
  <div class="card shadow-lg border-0 flex-grow-1" style="border-radius: 20px; overflow: hidden;">
    <div class="card-body p-4 overflow-auto">
      <h2 class="fw-bold mb-4">Live Readings</h2>

      <div class="row g-4">
        <div class="col-md-6 col-lg-3">
          <div class="card border-0 shadow-sm h-100" style="border-radius: 15px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
            <div class="card-body text-center text-white p-4">
              <h5 class="fw-bold mb-3">BVP</h5>
              <p class="display-3 fw-bold mb-2">{{ readings.bvp || '--' }}</p>
              <small class="opacity-75">Blood Volume Pulse</small>
            </div>
          </div>
        </div>

        <div class="col-md-6 col-lg-3">
          <div class="card border-0 shadow-sm h-100" style="border-radius: 15px; background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);">
            <div class="card-body text-center text-white p-4">
              <h5 class="fw-bold mb-3">Temperature</h5>
              <p class="display-3 fw-bold mb-2">{{ readings.temperature || '--' }}</p>
              <small class="opacity-75">°C</small>
            </div>
          </div>
        </div>

        <div class="col-md-6 col-lg-3">
          <div class="card border-0 shadow-sm h-100" style="border-radius: 15px; background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);">
            <div class="card-body text-center text-white p-4">
              <h5 class="fw-bold mb-3">EDA</h5>
              <p class="display-3 fw-bold mb-2">{{ readings.eda || '--' }}</p>
              <small class="opacity-75">µS</small>
            </div>
          </div>
        </div>

        <div class="col-md-6 col-lg-3">
          <div class="card border-0 shadow-sm h-100" style="border-radius: 15px; background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);">
            <div class="card-body text-center text-white p-4">
              <h5 class="fw-bold mb-3">Accelerometer</h5>
              <p class="fs-4 fw-bold mb-0" style="line-height: 1.6;">
                X: {{ readings.acc_x || '--' }}<br>
                Y: {{ readings.acc_y || '--' }}<br>
                Z: {{ readings.acc_z || '--' }}
              </p>
            </div>
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