<template>
  <div class="card shadow-lg border-0" style="flex: 1; border-radius: 20px; overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column h-100">
      <div class="d-flex justify-content-between align-items-center mb-3 pb-3 border-bottom">
        <h3 class="fw-bold mb-0">Sessions</h3>
        <button
            v-if="patient"
            class="btn btn-success btn-sm rounded-pill px-3"
            @click="$emit('create-session')">
          + New Session
        </button>
      </div>

      <!-- No patient selected -->
      <div v-if="!patient" class="flex-grow-1 d-flex align-items-center justify-content-center text-muted">
        <div class="text-center">
          <i class="bi bi-person-circle" style="font-size: 4rem;"></i>
          <p class="mt-3">Choose a patient to view sessions</p>
        </div>
      </div>

      <!-- Patient selected -->
      <div v-else class="flex-grow-1 d-flex flex-column">
        <!-- Patient Info -->
        <div class="card bg-light border-0 rounded-3 mb-3 p-3">
          <h5 class="fw-bold mb-2">{{ patient.full_name }}</h5>
          <div class="row small text-muted">
            <div class="col-6">
              <strong>ID:</strong> {{ patient.patient_id.slice(0, 8) }}...
            </div>
            <div class="col-6">
              <strong>Age:</strong> {{ patient.age }}
            </div>
            <div class="col-6">
              <strong>DOB:</strong> {{ patient.date_of_birth }}
            </div>
            <div class="col-6">
              <strong>Sessions:</strong> {{ sessions.length }}
            </div>
          </div>
        </div>

        <!-- Sessions List -->
        <div class="flex-grow-1 overflow-auto session-list">
          <div v-if="sessions.length === 0" class="text-center text-muted py-5">
            No sessions yet
          </div>
          <table v-else class="table table-hover mb-0">
            <thead class="sticky-top bg-white">
            <tr class="border-bottom">
              <th class="text-muted small fw-semibold py-2">SESSION</th>
              <th class="text-muted small fw-semibold py-2 text-end">STATUS</th>
              <th class="text-muted small fw-semibold py-2 text-end" style="width: 80px;"></th>
            </tr>
            </thead>
            <tbody>
            <tr
                v-for="session in sessions"
                :key="session.id"
                class="session-row"
                :class="{ 'table-primary': activeSessionId === session.id }">
              <td class="align-middle py-2">
                <div class="d-flex flex-column">
                  <span class="fw-medium">{{ session.session_name || 'Unnamed Session' }}</span>
                  <small class="text-muted" style="font-size: 0.75rem;">
                    {{ formatDate(session.started_at) }}
                  </small>
                  <small class="text-muted">
                    {{ session.reading_count }} readings
                    <span v-if="session.duration_seconds">
                        • {{ formatDuration(session.duration_seconds) }}
                      </span>
                  </small>
                </div>
              </td>
              <td class="align-middle py-2 text-end">
                  <span class="badge rounded-pill" :class="session.is_active ? 'bg-success' : 'bg-secondary'">
                    {{ session.is_active ? 'Active' : 'Ended' }}
                  </span>
              </td>
              <td class="align-middle py-2 text-end">
                <div class="d-flex gap-1 justify-content-end">
                  <button
                      class="btn btn-sm btn-primary rounded-pill px-2 set-active-btn"
                      @click="$emit('set-active-session', session.id)"
                      :disabled="activeSessionId === session.id"
                      title="Set as active">
                    {{ activeSessionId === session.id ? '✓' : '○' }}
                  </button>
                  <button
                      class="btn btn-sm delete-btn"
                      @click="$emit('delete-session', session)"
                      title="Delete session">
                    ×
                  </button>
                </div>
              </td>
            </tr>
            </tbody>
          </table>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    patient: Object,
    sessions: Array,
    activeSessionId: Number  // keep this for highlighting
  },
  methods: {
    formatDate(dateString) {
      return new Date(dateString).toLocaleString();
    },

    formatDuration(seconds) {
      const mins = Math.floor(seconds / 60);
      const secs = Math.floor(seconds % 60);
      return `${mins}m ${secs}s`;
    }
  }
}
</script>

<style scoped>
.session-list {
  overflow-y: auto;
  max-height: 100%;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.session-list::-webkit-scrollbar {
  display: none;
}

.table thead th {
  border-top: none;
  border-bottom: 2px solid #dee2e6;
  letter-spacing: 0.5px;
}

.session-row {
  transition: background-color 0.15s ease;
}

.session-row:hover {
  background-color: #f8f9fa;
}

.table-primary {
  background-color: #cfe2ff !important;
}

.session-row td {
  border-bottom: 1px solid #f0f0f0;
}

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s, transform 0.2s;
  text-decoration: none;
  background-color: #dc3545;
  color: white;
  width: 24px;
  height: 24px;
  border-radius: 50%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border: none;
}

.session-row:hover .delete-btn {
  opacity: 1;
}

.delete-btn:hover {
  background-color: #c82333 !important;
  transform: scale(1.1);
}

.set-active-btn {
  font-size: 0.75rem;
  min-width: 28px;
}

.set-active-btn:disabled {
  opacity: 1;
  background-color: #198754;
  border-color: #198754;
}
</style>