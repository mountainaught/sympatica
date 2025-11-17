// components/session/SessionPage.vue
<template>
  <div class="flex-grow-1 d-flex flex-column gap-3">
    <div class="card shadow-lg border-0 rounded-card">
      <div class="card-body p-4">
        <div class="d-flex justify-content-between align-items-center mb-3">
          <div>
            <h2 class="fw-bold mb-1">{{ sessionData?.session_name || 'Session Graphs' }}</h2>
            <p class="text-muted mb-0">{{ sessionData?.patient_name }}</p>
          </div>
          <button class="btn btn-secondary rounded-pill" @click="$router.go(-1)">
            <i class="bi bi-arrow-left me-2"></i>Back
          </button>
        </div>
      </div>
    </div>

    <div class="card shadow-lg border-0 flex-grow-1 rounded-card session-graphs-card">
      <LoadingSpinner :show="loading" message="Loading session data..." />

      <div v-if="!hasData" class="col-12">
        <div class="empty-state">
          <i class="bi bi-graph-down empty-state-icon"></i>
          <h5 class="empty-state-title">No data recorded</h5>
          <p class="empty-state-text">
            This session doesn't have any recorded data yet. Start recording to see graphs here.
          </p>
          <button class="btn btn-primary rounded-pill px-4 empty-state-action" @click="$router.push({ path: '/home', query: $route.query })">
            <i class="bi bi-arrow-left me-2"></i>Go to Home
          </button>
        </div>
      </div>

      <template v-else>
        <!-- Graph Grid -->
        <div class="d-flex flex-column gap-3 flex-grow-1 p-3" style="min-height: 0;">
          <!-- Top Row -->
          <div class="row g-3 flex-grow-1" style="min-height: 0;">
            <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
              <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column" style="min-height: 0;">
                <div class="graph-header mb-2">
                  <span class="fw-semibold text-dark">BVP</span>
                  <span class="text-muted small ms-2">Blood Volume Pulse</span>
                </div>
                <div ref="bvpGraph" class="flex-grow-1" style="min-height: 0;"></div>
              </div>
            </div>

            <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
              <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column" style="min-height: 0;">
                <div class="graph-header mb-2">
                  <span class="fw-semibold text-dark">EDA</span>
                  <span class="text-muted small ms-2">Electrodermal Activity</span>
                </div>
                <div ref="edaGraph" class="flex-grow-1" style="min-height: 0;"></div>
              </div>
            </div>
          </div>

          <!-- Bottom Row -->
          <div class="row g-3 flex-grow-1" style="min-height: 0;">
            <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
              <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column" style="min-height: 0;">
                <div class="graph-header mb-2">
                  <span class="fw-semibold text-dark">Temperature</span>
                  <span class="text-muted small ms-2">Skin Surface</span>
                </div>
                <div ref="tempGraph" class="flex-grow-1" style="min-height: 0;"></div>
              </div>
            </div>

            <div class="col-md-6 d-flex flex-column" style="min-height: 0;">
              <div class="graph-container bg-white border rounded-3 p-3 flex-grow-1 d-flex flex-column" style="min-height: 0;">
                <div class="graph-header mb-2">
                  <span class="fw-semibold text-dark">Acceleration</span>
                  <span class="text-muted small ms-2">3-Axis Motion</span>
                </div>
                <div ref="accGraph" class="flex-grow-1" style="min-height: 0;"></div>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script>
import Plotly from 'plotly.js-dist-min';
import { fetchAPI } from '../../utils/helpers.js';
import LoadingSpinner from '../../utils/Loading.vue';

export default {
  components: {
    LoadingSpinner
  },
  data() {
    return {
      sessionData: null,
      loading: true
    }
  },
  computed: {
    sessionId() {
      return this.$route.query.uuid;
    },
    hasData() {
      if (!this.sessionData?.readings) return false;
      const { bvp, eda, temperature, acc } = this.sessionData.readings;
      return (bvp?.length > 0 || eda?.length > 0 || temperature?.length > 0 || acc?.length > 0);
    }
  },
  async mounted() {
    await this.loadSessionData();
  },
  methods: {
    async loadSessionData() {
      try {
        const data = await fetchAPI(`/sessions/${this.sessionId}/readings/`);
        this.sessionData = data;

        this.$nextTick(() => {
          this.renderGraphs(data.readings);
        });
      } catch (error) {
        console.error('Error loading session data:', error);
        window.$toast.addToast({
          title: 'Load Failed',
          message: 'Failed to load session data',
          type: 'error'
        });
      } finally {
        this.loading = false;
      }
    },

    renderGraphs(readings) {
      const layout = {
        margin: { t: 20, r: 20, b: 40, l: 60 },
        xaxis: { title: 'Time' },
        dragmode: 'zoom',
        hovermode: 'x unified'
      };

      const config = {
        responsive: true,
        displayModeBar: true,
        modeBarButtonsToRemove: ['lasso2d', 'select2d']
      };

      // BVP
      const bvpData = readings.bvp.map(r => ({
        x: new Date(r.timestamp),
        y: parseFloat(r.value)
      }));

      Plotly.newPlot(this.$refs.bvpGraph, [{
        x: bvpData.map(d => d.x),
        y: bvpData.map(d => d.y),
        type: 'scatter',
        mode: 'lines',
        line: { color: '#dc3545', width: 1 }
      }], { ...layout, yaxis: { title: 'BVP' } }, config);

      // EDA
      const edaData = readings.eda.map(r => ({
        x: new Date(r.timestamp),
        y: parseFloat(r.value)
      }));

      Plotly.newPlot(this.$refs.edaGraph, [{
        x: edaData.map(d => d.x),
        y: edaData.map(d => d.y),
        type: 'scatter',
        mode: 'lines',
        line: { color: '#0d6efd', width: 1 }
      }], { ...layout, yaxis: { title: 'µS' } }, config);

      // Temperature
      const tempData = readings.temperature.map(r => ({
        x: new Date(r.timestamp),
        y: parseFloat(r.value)
      }));

      Plotly.newPlot(this.$refs.tempGraph, [{
        x: tempData.map(d => d.x),
        y: tempData.map(d => d.y),
        type: 'scatter',
        mode: 'lines',
        line: { color: '#fd7e14', width: 1 }
      }], { ...layout, yaxis: { title: '°C' } }, config);

      // Accelerometer
      const accData = readings.acc.map(r => {
        const acc = JSON.parse(r.value);
        return {
          x: new Date(r.timestamp),
          x_val: acc.x,
          y_val: acc.y,
          z_val: acc.z
        };
      });

      Plotly.newPlot(this.$refs.accGraph, [
        {
          x: accData.map(d => d.x),
          y: accData.map(d => d.x_val),
          type: 'scatter',
          mode: 'lines',
          name: 'X',
          line: { color: '#198754', width: 1 }
        },
        {
          x: accData.map(d => d.x),
          y: accData.map(d => d.y_val),
          type: 'scatter',
          mode: 'lines',
          name: 'Y',
          line: { color: '#20c997', width: 1 }
        },
        {
          x: accData.map(d => d.x),
          y: accData.map(d => d.z_val),
          type: 'scatter',
          mode: 'lines',
          name: 'Z',
          line: { color: '#0dcaf0', width: 1 }
        }
      ], { ...layout, yaxis: { title: 'g-force' } }, config);
    }
  }
}
</script>