<template>
  <div class="card shadow-lg border-0" style="flex: 1; border-radius: 20px; overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column">
      <h3 class="fw-bold mb-3">Sessions</h3>

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
        <div class="flex-grow-1 overflow-auto">
          <div v-if="sessions.length === 0" class="text-center text-muted py-5">
            No sessions yet
          </div>
          <div v-else class="list-group list-group-flush">
            <div
                v-for="session in sessions"
                :key="session.id"
                class="list-group-item border-0 rounded-3 mb-2 bg-light">
              <div class="d-flex justify-content-between align-items-start">
                <div>
                  <h6 class="mb-1">{{ session.session_name || 'Unnamed Session' }}</h6>
                  <small class="text-muted">
                    {{ formatDate(session.started_at) }}
                  </small>
                  <br>
                  <small class="text-muted">
                    {{ session.reading_count }} readings
                    <span v-if="session.duration_seconds">
                      • {{ formatDuration(session.duration_seconds) }}
                    </span>
                  </small>
                </div>
                <span class="badge rounded-pill" :class="session.is_active ? 'bg-success' : 'bg-secondary'">
                  {{ session.is_active ? 'Active' : 'Ended' }}
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    patient: Object,
    sessions: Array
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