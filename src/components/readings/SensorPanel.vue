<template>
  <div class="card flex flex-col h-96">
    <div class="border-b border-default p-4 flex items-center justify-between">
      <div class="flex items-center gap-3">
        <i :class="['bi', icon, 'text-xl']" :style="{ color: color }"></i>
        <div>
          <h3 class="font-semibold text-sm text-primary tracking-wide">{{ title }}</h3>
          <p class="text-xs text-muted uppercase mt-0.5">{{ isMultiAxis ? 'X, Y, Z Axes' : 'Live Data' }}</p>
        </div>
      </div>
      <div class="text-right">
        <div v-if="!isMultiAxis" class="flex items-baseline gap-2">
          <span class="text-3xl font-bold text-primary tabular-nums">{{ displayValue }}</span>
          <span class="text-sm text-muted">{{ unit }}</span>
        </div>
        <div v-else class="text-xs space-y-1 tabular-nums">
          <div class="flex justify-between gap-3">
            <span class="text-muted">X:</span>
            <span class="font-semibold text-primary">{{ value.x !== null ? value.x.toFixed(2) : '--' }}</span>
          </div>
          <div class="flex justify-between gap-3">
            <span class="text-muted">Y:</span>
            <span class="font-semibold text-primary">{{ value.y !== null ? value.y.toFixed(2) : '--' }}</span>
          </div>
          <div class="flex justify-between gap-3">
            <span class="text-muted">Z:</span>
            <span class="font-semibold text-primary">{{ value.z !== null ? value.z.toFixed(2) : '--' }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="flex-1 p-4 bg-app min-h-0">
      <div ref="graphContainer" class="w-full h-full bg-surface border border-default"></div>
    </div>

    <div class="border-t border-default px-4 py-2 bg-app">
      <div class="flex items-center justify-between text-xs">
        <span class="text-muted">{{ statusText }}</span>
        <span class="text-muted font-mono">{{ samplingRate }}</span>
      </div>
    </div>
  </div>
</template>

<script>
import uPlot from 'uplot'
import 'uplot/dist/uPlot.min.css'

export default {
  props: {
    title: String,
    unit: String,
    value: [Number, Object],
    color: String,
    icon: String,
    sensorType: String,
    isMultiAxis: Boolean
  },
  data() {
    return {
      chart: null,
      graphData: {
        times: [],
        values: [],
        x: [],
        y: [],
        z: []
      },
      timeWindow: 60,
      startTime: null
    }
  },
  computed: {
    displayValue() {
      if (this.value === null) return '--'
      return typeof this.value === 'number' ? this.value.toFixed(2) : '--'
    },
    statusText() {
      if (this.value === null) return 'NO DATA'
      return 'ACTIVE'
    },
    samplingRate() {
      const rates = {
        'bvp': '64 Hz',
        'eda': '4 Hz',
        'temperature': '4 Hz',
        'acc': '32 Hz'
      }
      return rates[this.sensorType] || ''
    }
  },
  mounted() {
    this.startTime = Date.now() / 1000
    this.$nextTick(() => {
      this.initChart()
    })
    window.addEventListener('resize', this.handleResize)
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.handleResize)
    if (this.chart) {
      this.chart.destroy()
    }
  },
  methods: {
    initChart() {
      const opts = {
        width: this.$refs.graphContainer.clientWidth,
        height: this.$refs.graphContainer.clientHeight,
        cursor: { drag: { x: false, y: false }, lock: true },
        legend: { show: false },
        scales: { x: { time: false } },
        axes: [
          {
            label: 'Time (s)',
            labelSize: 16,
            size: 35,
            grid: { show: true, stroke: '#e2e8f0', width: 1 }
          },
          {
            label: this.unit,
            labelSize: 16,
            size: 45,
            grid: { show: true, stroke: '#e2e8f0', width: 1 }
          }
        ]
      }

      if (this.isMultiAxis) {
        this.chart = new uPlot({
          ...opts,
          series: [
            {},
            { label: 'X', stroke: 'var(--color-sensor-acc-x)', width: 1.5 },
            { label: 'Y', stroke: 'var(--color-sensor-acc-y)', width: 1.5 },
            { label: 'Z', stroke: 'var(--color-sensor-acc-z)', width: 1.5 }
          ]
        }, [[0], [0], [0], [0]], this.$refs.graphContainer)
      } else {
        this.chart = new uPlot({
          ...opts,
          series: [
            {},
            { label: this.title, stroke: this.color, width: 2 }
          ]
        }, [[0], [0]], this.$refs.graphContainer)
      }
    },

    addDataPoint(value, timestamp) {
      const time = (new Date(timestamp).getTime() / 1000) - this.startTime

      if (this.isMultiAxis) {
        this.graphData.times.push(time)
        this.graphData.x.push(value.x)
        this.graphData.y.push(value.y)
        this.graphData.z.push(value.z)

        const cutoff = time - this.timeWindow
        while (this.graphData.times.length > 0 && this.graphData.times[0] < cutoff) {
          this.graphData.times.shift()
          this.graphData.x.shift()
          this.graphData.y.shift()
          this.graphData.z.shift()
        }

        if (this.chart) {
          this.chart.setData([
            this.graphData.times,
            this.graphData.x,
            this.graphData.y,
            this.graphData.z
          ])
        }
      } else {
        this.graphData.times.push(time)
        this.graphData.values.push(value)

        const cutoff = time - this.timeWindow
        while (this.graphData.times.length > 0 && this.graphData.times[0] < cutoff) {
          this.graphData.times.shift()
          this.graphData.values.shift()
        }

        if (this.chart) {
          this.chart.setData([this.graphData.times, this.graphData.values])
        }
      }
    },

    handleResize() {
      if (this.chart && this.$refs.graphContainer) {
        this.chart.setSize({
          width: this.$refs.graphContainer.clientWidth,
          height: this.$refs.graphContainer.clientHeight
        })
      }
    }
  }
}
</script>