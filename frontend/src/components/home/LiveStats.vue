// components/home/LiveStats.vue
<template>
  <div class="card shadow-lg border-0 flex-grow-1 rounded-card" style="overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column h-100">
      <!-- HEADER -->
      <div class="d-flex justify-content-between align-items-center mb-4 pb-3 border-bottom">
        <h2 class="fw-bold mb-0 text-dark">Physiological Monitoring</h2>
        <div class="d-flex gap-2 align-items-center">
          <span class="badge bg-success px-3 py-2" v-if="isRecording">
            <i class="bi bi-circle-fill blink me-2"></i>RECORDING
          </span>
          <span class="text-muted small">{{ currentTime }}</span>
        </div>
      </div>

      <!-- VITALS CARDS -->
      <VitalCards :readings="readings" />

      <!-- LIVE GRAPHS -->
      <LiveGraphs :readings="readings" />
    </div>
  </div>
</template>

<script>
import VitalCards from './readings/VitalCards.vue';
import LiveGraphs from './readings/LiveGraphs.vue';
import DataParser from '../../services/DataParser.js';

export default {
  components: {
    VitalCards,
    LiveGraphs
  },
  props: {
    isRecording: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      readings: {
        bvp: null,
        temperature: null,
        eda: null,
        acc_x: null,
        acc_y: null,
        acc_z: null
      },
      currentTime: ''
    }
  },
  computed: {
    activeSessionId() {
      return this.$route.query.session || null;
    }
  },
  watch: {
    activeSessionId: {
      immediate: true,
      handler(newSessionId) {
        if (newSessionId) {
          DataParser.setSession(newSessionId);
        } else {
          DataParser.setSession(null);
        }
      }
    }
  },
  mounted() {
    DataParser.onData(this.handleReading);
    this.updateTime();
    setInterval(this.updateTime, 1000);
  },
  methods: {
    updateTime() {
      const now = new Date();
      this.currentTime = now.toLocaleTimeString();
    },

    handleReading(reading) {
      switch(reading.type) {
        case 'bvp':
          this.readings.bvp = reading.value.toFixed(2);
          break;
        case 'temperature':
          this.readings.temperature = reading.value.toFixed(1);
          break;
        case 'eda':
          this.readings.eda = reading.value.toFixed(2);
          break;
        case 'acc':
          this.readings.acc_x = reading.value.x.toFixed(2);
          this.readings.acc_y = reading.value.y.toFixed(2);
          this.readings.acc_z = reading.value.z.toFixed(2);
          break;
      }
    }
  }
}
</script>