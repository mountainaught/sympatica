// SessionsPanel.vue - WITH ACCORDION
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
      <div v-if="!patient" class="empty-state flex-grow-1">
        <i class="bi bi-person-circle empty-state-icon"></i>
        <h5 class="empty-state-title">No patient selected</h5>
        <p class="empty-state-text">
          Select a patient from the list to view their sessions
        </p>
      </div>

      <!-- Patient selected -->
      <div v-else class="flex-grow-1 d-flex flex-column">
        <!-- Patient Info Accordion -->
        <div class="accordion mb-3" id="patientInfoAccordion">
          <div class="accordion-item border-0 rounded-3 bg-light">
            <h2 class="accordion-header">
              <button
                  class="accordion-button bg-light rounded-3 fw-bold"
                  :class="{ collapsed: !isAccordionOpen }"
                  type="button"
                  @click="isAccordionOpen = !isAccordionOpen"
                  aria-controls="patientInfoCollapse">
                {{ patient.full_name }}
              </button>
            </h2>
            <transition name="accordion">
              <div
                  v-show="isAccordionOpen"
                  id="patientInfoCollapse"
                  class="accordion-collapse">
                <div class="accordion-body pt-2">
                  <div class="row small text-muted">
                    <div class="col-6">
                      <strong>ID:</strong> {{ patient.patient_id }}
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
              </div>
            </transition>
          </div>
        </div>

        <!-- Sessions List -->
        <div class="flex-grow-1 overflow-auto hide-scrollbar">
          <!-- Empty State for Sessions -->
          <div v-if="sessions.length === 0" class="empty-state" style="min-height: 300px;">
            <i class="bi bi-clipboard-data empty-state-icon"></i>
            <h5 class="empty-state-title">No sessions yet</h5>
            <p class="empty-state-text">
              Click the "+ New Session" button above to start recording physiological data
            </p>
          </div>

          <!-- Sessions Table -->
          <table v-else class="table table-hover table-custom mb-0">
            <thead class="sticky-top bg-white">
            <tr class="border-bottom">
              <th class="text-muted small fw-semibold py-2">SESSION</th>
              <th class="text-muted small fw-semibold py-2 text-end">READINGS</th>
              <th class="text-muted small fw-semibold py-2 text-end" style="width: 180px;"></th>
            </tr>
            </thead>
            <tbody>
            <tr
                v-for="session in sessions"
                :key="session.session_id"
                class="hover-parent"
                :class="{ 'table-primary': activeSessionId === session.session_id }">
              <td class="align-middle py-2">
                <div class="d-flex flex-column">
                  <span class="fw-medium">{{ session.session_name || 'Unnamed Session' }}</span>
                  <small class="text-muted font-mono-small">
                    {{ formatDate(session.started_at) }}
                  </small>
                </div>
              </td>
              <td class="align-middle py-2 text-end">
                <span class="badge bg-light text-dark rounded-pill px-3">
                  <i class="bi bi-activity me-1"></i>
                  {{ session.reading_count }}
                </span>
              </td>
              <td class="align-middle py-2 text-end">
                <div class="d-flex gap-2 justify-content-end align-items-center">
                  <button
                      class="btn btn-primary btn-sm set-active-btn"
                      @click="setActiveSession(session.session_id)"
                      :disabled="activeSessionId === session.session_id"
                      title="Set as active">
                    <i v-if="activeSessionId === session.session_id" class="bi bi-check-lg"></i>
                    <i v-else class="bi bi-circle"></i>
                  </button>
                  <button
                      class="btn btn-info btn-sm rounded-pill px-3"
                      @click="viewGraphs(session.session_id)"
                      title="View graphs">
                    <i class="bi bi-graph-up"></i>
                  </button>
                  <button
                      class="btn btn-danger btn-sm delete-btn-circular hover-reveal"
                      @click="$emit('delete-session', session)"
                      title="Delete session">
                    <i class="bi bi-x-lg"></i>
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
import { formatDate, truncateId } from '../../utils/helpers.js';

export default {
  props: {
    patient: Object,
    sessions: Array,
    activeSessionId: String
  },
  data() {
    return {
      isAccordionOpen: false
    }
  },
  methods: {
    formatDate,
    truncateId,
    setActiveSession(sessionId) {
      this.$router.push({ query: { session: sessionId } });
    },
    viewGraphs(sessionId) {
      this.$router.push({ path: '/session', query: { uuid: sessionId } });
    }
  }
}
</script>