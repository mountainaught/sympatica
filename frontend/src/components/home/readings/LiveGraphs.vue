// components/home/LiveGraphs.vue
<template>
  <div class="flex-grow-1 d-flex flex-column" style="min-height: 0;">
    <div class="d-flex justify-content-between align-items-center mb-3">
      <h5 class="fw-semibold mb-0 text-dark">Real-Time Waveforms</h5>
      <button class="btn btn-sm btn-outline-danger rounded-pill px-3" @click="clearAllGraphs">
        <i class="bi bi-arrow-clockwise me-1"></i>Clear All
      </button>
    </div>

    <!-- Graph Grid -->
    <div class="row g-3 flex-grow-1" style="min-height: 0;">
      <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
        <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column">
          <div class="graph-header mb-2">
            <span class="fw-semibold text-dark">BVP</span>
            <span class="text-muted small ms-2">Blood Volume Pulse</span>
          </div>
          <div ref="bvpGraph" class="flex-grow-1"></div>
        </div>
      </div>

      <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
        <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column">
          <div class="graph-header mb-2">
            <span class="fw-semibold text-dark">EDA</span>
            <span class="text-muted small ms-2">Electrodermal Activity</span>
          </div>
          <div ref="edaGraph" class="flex-grow-1"></div>
        </div>
      </div>

      <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
        <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column">
          <div class="graph-header mb-2">
            <span class="fw-semibold text-dark">Temperature</span>
            <span class="text-muted small ms-2">Skin Surface</span>
          </div>
          <div ref="tempGraph" class="flex-grow-1"></div>
        </div>
      </div>

      <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
        <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column">
          <div class="graph-header mb-2">
            <span class="fw-semibold text-dark">Acceleration</span>
            <span class="text-muted small ms-2">3-Axis Motion</span>
          </div>
          <div ref="accGraph" class="flex-grow-1"></div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import uPlot from 'uplot';
import 'uplot/dist/uPlot.min.css';

export default {
  props: {
    readings: {
      type: Object,
      required: true
    }
  },
  data() {
    return {
      charts: {
        bvp: null,
        eda: null,
        temp: null,
        acc: null
      },
      graphData: {
        bvp: { times: [], values: [] },
        eda: { times: [], values: [] },
        temp: { times: [], values: [] },
        acc: { times: [], x: [], y: [], z: [] }
      },
      timeWindow: 60,
      startTime: null
    }
  },
  watch: {
    'readings.bvp'(newVal) {
      if (newVal !== null) {
        this.addDataPoint('bvp', parseFloat(newVal));
      }
    },
    'readings.eda'(newVal) {
      if (newVal !== null) {
        this.addDataPoint('eda', parseFloat(newVal));
      }
    },
    'readings.temperature'(newVal) {
      if (newVal !== null) {
        this.addDataPoint('temp', parseFloat(newVal));
      }
    },
    'readings.acc_x'(newVal) {
      if (newVal !== null && this.readings.acc_y !== null && this.readings.acc_z !== null) {
        this.addAccDataPoint({
          x: parseFloat(this.readings.acc_x),
          y: parseFloat(this.readings.acc_y),
          z: parseFloat(this.readings.acc_z)
        });
      }
    }
  },
  mounted() {
    this.startTime = Date.now() / 1000;
    this.$nextTick(() => {
      this.initializeCharts();
    });
    window.addEventListener('resize', this.handleResize);
  },
  beforeUnmount() {
    window.removeEventListener('resize', this.handleResize);
    Object.values(this.charts).forEach(chart => {
      if (chart) chart.destroy();
    });
  },
  methods: {
    getRelativeTime() {
      return (Date.now() / 1000) - this.startTime;
    },

    initializeCharts() {
      const baseOpts = {
        width: this.$refs.bvpGraph.clientWidth,
        height: this.$refs.bvpGraph.clientHeight,
        cursor: {
          drag: { x: false, y: false },
          lock: true
        },
        legend: { show: false },
        scales: {
          x: { time: false }
        },
        axes: [
          {
            label: 'Time (s)',
            labelSize: 20,
            size: 40,
            grid: { show: true, stroke: '#e9ecef', width: 1 }
          }
        ]
      };

      // BVP Chart
      this.charts.bvp = new uPlot({
        ...baseOpts,
        width: this.$refs.bvpGraph.clientWidth,
        height: this.$refs.bvpGraph.clientHeight,
        series: [
          {},
          {
            label: 'BVP',
            stroke: '#dc3545',
            width: 2
          }
        ],
        axes: [
          baseOpts.axes[0],
          {
            label: 'BVP',
            labelSize: 20,
            size: 50,
            grid: { show: true, stroke: '#e9ecef', width: 1 }
          }
        ]
      }, [[0], [0]], this.$refs.bvpGraph);

      // EDA Chart
      this.charts.eda = new uPlot({
        ...baseOpts,
        width: this.$refs.edaGraph.clientWidth,
        height: this.$refs.edaGraph.clientHeight,
        series: [
          {},
          {
            label: 'EDA',
            stroke: '#0d6efd',
            width: 2
          }
        ],
        axes: [
          baseOpts.axes[0],
          {
            label: 'µS',
            labelSize: 20,
            size: 50,
            grid: { show: true, stroke: '#e9ecef', width: 1 }
          }
        ]
      }, [[0], [0]], this.$refs.edaGraph);

      // Temperature Chart
      this.charts.temp = new uPlot({
        ...baseOpts,
        width: this.$refs.tempGraph.clientWidth,
        height: this.$refs.tempGraph.clientHeight,
        series: [
          {},
          {
            label: 'Temp',
            stroke: '#fd7e14',
            width: 2
          }
        ],
        axes: [
          baseOpts.axes[0],
          {
            label: '°C',
            labelSize: 20,
            size: 50,
            grid: { show: true, stroke: '#e9ecef', width: 1 }
          }
        ]
      }, [[0], [0]], this.$refs.tempGraph);

      // Accelerometer Chart (3 series)
      this.charts.acc = new uPlot({
        ...baseOpts,
        width: this.$refs.accGraph.clientWidth,
        height: this.$refs.accGraph.clientHeight,
        series: [
          {},
          {
            label: 'X',
            stroke: '#198754',
            width: 1.5
          },
          {
            label: 'Y',
            stroke: '#20c997',
            width: 1.5
          },
          {
            label: 'Z',
            stroke: '#0dcaf0',
            width: 1.5
          }
        ],
        axes: [
          baseOpts.axes[0],
          {
            label: 'g-force',
            labelSize: 20,
            size: 50,
            grid: { show: true, stroke: '#e9ecef', width: 1 }
          }
        ],
        legend: { show: true }
      }, [[0], [0], [0], [0]], this.$refs.accGraph);
    },

    addDataPoint(type, value) {
      const now = this.getRelativeTime();
      const data = this.graphData[type];

      data.times.push(now);
      data.values.push(value);

      const cutoff = now - this.timeWindow;
      while (data.times.length > 0 && data.times[0] < cutoff) {
        data.times.shift();
        data.values.shift();
      }

      if (this.charts[type]) {
        this.charts[type].setData([data.times, data.values]);
      }
    },

    addAccDataPoint(value) {
      const now = this.getRelativeTime();
      const data = this.graphData.acc;

      data.times.push(now);
      data.x.push(value.x);
      data.y.push(value.y);
      data.z.push(value.z);

      const cutoff = now - this.timeWindow;
      while (data.times.length > 0 && data.times[0] < cutoff) {
        data.times.shift();
        data.x.shift();
        data.y.shift();
        data.z.shift();
      }

      if (this.charts.acc) {
        this.charts.acc.setData([data.times, data.x, data.y, data.z]);
      }
    },

    clearAllGraphs() {
      this.graphData.bvp = { times: [], values: [] };
      this.graphData.eda = { times: [], values: [] };
      this.graphData.temp = { times: [], values: [] };
      this.graphData.acc = { times: [], x: [], y: [], z: [] };

      this.startTime = Date.now() / 1000;

      if (this.charts.bvp) this.charts.bvp.setData([[0], [0]]);
      if (this.charts.eda) this.charts.eda.setData([[0], [0]]);
      if (this.charts.temp) this.charts.temp.setData([[0], [0]]);
      if (this.charts.acc) this.charts.acc.setData([[0], [0], [0], [0]]);
    },

    handleResize() {
      Object.keys(this.charts).forEach(key => {
        if (this.charts[key] && this.$refs[`${key}Graph`]) {
          this.charts[key].setSize({
            width: this.$refs[`${key}Graph`].clientWidth,
            height: this.$refs[`${key}Graph`].clientHeight
          });
        }
      });
    }
  }
}
</script>