<template>
  <div class="card shadow-lg border-0 rounded-card" style="flex: 1; overflow: hidden;">
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
              <strong>ID:</strong> {{ truncateId(patient.patient_id) }}...
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
        <div class="flex-grow-1 overflow-auto hide-scrollbar">
          <div v-if="sessions.length === 0" class="text-center text-muted py-5">
            No sessions yet
          </div>
          <table v-else class="table table-hover table-custom mb-0">
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
                class="hover-parent"
                :class="{ 'table-primary': activeSessionId === session.id }">
              <td class="align-middle py-2">
                <div class="d-flex flex-column">
                  <span class="fw-medium">{{ session.session_name || 'Unnamed Session' }}</span>
                  <small class="text-muted font-mono-small">
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
                      class="btn btn-sm btn-primary rounded-pill px-2 min-width-btn"
                      @click="$emit('set-active-session', session.id)"
                      :disabled="activeSessionId === session.id"
                      title="Set as active">
                    {{ activeSessionId === session.id ? '✓' : '○' }}
                  </button>
                  <button
                      class="btn btn-danger btn-sm delete-btn-circular hover-reveal"
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
import { formatDate, formatDuration, truncateId } from '../../utils/helpers.js';

export default {
  props: {
    patient: Object,
    sessions: Array,
    activeSessionId: Number
  },
  methods: {
    formatDate,
    formatDuration,
    truncateId
  }
}
</script>