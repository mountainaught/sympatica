<template>
  <div class="h-full bg-app p-6 overflow-auto">
    <div class="bg-surface border border-default p-4 mb-6">
      <div class="flex items-center justify-between">
        <div>
          <h1 class="text-2xl font-bold text-primary tracking-tight">PHYSIOLOGICAL MONITORING</h1>
          <p class="text-sm text-secondary mt-1">Real-time biometric data acquisition</p>
        </div>
        <div class="flex items-center gap-4">
          <div v-if="isRecording" class="flex items-center gap-2 px-4 py-2 bg-[var(--color-danger)] text-inverse">
            <div class="w-2 h-2 bg-white animate-pulse"></div>
            <span class="text-sm font-semibold uppercase tracking-wide">RECORDING</span>
          </div>
          <span class="text-sm text-muted font-mono">{{ currentTime }}</span>
        </div>
      </div>
    </div>

    <div class="grid grid-cols-2 gap-6">
      <SensorPanel
          ref="bvpPanel"
          title="BLOOD VOLUME PULSE"
          unit="units"
          :value="readings.bvp"
          :color="'var(--color-sensor-bvp)'"
          icon="bi-heart-pulse"
          sensor-type="bvp"
      />

      <SensorPanel
          ref="edaPanel"
          title="ELECTRODERMAL ACTIVITY"
          unit="µS"
          :value="readings.eda"
          :color="'var(--color-sensor-eda)'"
          icon="bi-droplet"
          sensor-type="eda"
      />

      <SensorPanel
          ref="tempPanel"
          title="SKIN TEMPERATURE"
          unit="°C"
          :value="readings.temperature"
          :color="'var(--color-sensor-temp)'"
          icon="bi-thermometer-half"
          sensor-type="temperature"
      />

      <SensorPanel
          ref="accPanel"
          title="ACCELEROMETER"
          unit="g"
          :value="readings.acc"
          :color="'var(--color-sensor-acc-x)'"
          icon="bi-shuffle"
          sensor-type="acc"
          :is-multi-axis="true"
      />
    </div>
  </div>
</template>

<script>
import { listen } from '@tauri-apps/api/event'
import SensorPanel from './SensorPanel.vue'

export default {
  components: {
    SensorPanel
  },
  props: {
    isStreaming: {
      type: Boolean,
      default: false
    }
  },
  data() {
    return {
      isRecording: false,
      currentTime: '',
      readings: {
        bvp: null,
        eda: null,
        temperature: null,
        acc: { x: null, y: null, z: null }
      },
      unlisten: null
    }
  },
  async mounted() {
    this.updateTime()
    setInterval(this.updateTime, 1000)

    this.unlisten = await listen('sensor-data', (event) => {
      this.handleSensorData(event.payload)
    })
  },
  beforeUnmount() {
    if (this.unlisten) {
      this.unlisten()
    }
  },
  methods: {
    updateTime() {
      const now = new Date()
      this.currentTime = now.toLocaleTimeString('en-US', { hour12: false })
    },

    handleSensorData(data) {
      const { sensor_type, value, timestamp } = data

      switch(sensor_type) {
        case 'bvp':
          this.readings.bvp = value
          if (this.$refs.bvpPanel) {
            this.$refs.bvpPanel.addDataPoint(value, timestamp)
          }
          break
        case 'eda':
          this.readings.eda = value
          if (this.$refs.edaPanel) {
            this.$refs.edaPanel.addDataPoint(value, timestamp)
          }
          break
        case 'temperature':
          this.readings.temperature = value
          if (this.$refs.tempPanel) {
            this.$refs.tempPanel.addDataPoint(value, timestamp)
          }
          break
        case 'acc':
          this.readings.acc = value
          if (this.$refs.accPanel) {
            this.$refs.accPanel.addDataPoint(value, timestamp)
          }
          break
      }
    }
  }
}
</script>