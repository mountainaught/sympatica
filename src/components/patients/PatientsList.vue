// PatientsList.vue
// components/patients/PatientsList.vue
<template>
  <div class="card shadow-lg border-0 rounded-card" style="flex: 1; overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column h-100">
      <!-- Only show header when there are patients -->
      <div class="d-flex justify-content-between align-items-center mb-3 pb-3">
        <h3 class="fw-bold mb-0">Patients</h3>
        <!-- Hide button when empty -->
        <button v-if="patients.length > 0" class="btn btn-primary btn-sm rounded-pill px-3" @click="$emit('create-patient')">
          + New Patient
        </button>
      </div>

      <div class="flex-grow-1 overflow-auto hide-scrollbar">
        <!-- Empty State -->
        <div v-if="patients.length === 0" class="empty-state">
          <i class="bi bi-person-plus empty-state-icon"></i>
          <h5 class="empty-state-title">No patients yet</h5>
          <p class="empty-state-text">
            Get started by creating your first patient record
          </p>
          <button class="btn btn-primary rounded-pill px-4 empty-state-action" @click="$emit('create-patient')">
            <i class="bi bi-plus-lg me-2"></i>Create Patient
          </button>
        </div>

        <!-- Patients Table -->
        <table v-else class="table table-hover table-custom mb-0">
          <thead class="sticky-top bg-white">
          <tr class="border-bottom">
            <th class="text-muted small fw-semibold py-2">NAME</th>
            <th class="text-muted small fw-semibold py-2 text-end">SESSIONS</th>
            <th class="text-muted small fw-semibold py-2 text-end" style="width: 40px;"></th>
          </tr>
          </thead>
          <tbody>
          <tr
              v-for="patient in patients"
              :key="patient.patient_id"
              class="hover-parent"
              :class="{ 'table-active': selectedPatientId === patient.patient_id }"
              @click="$emit('select-patient', patient)"
              style="cursor: pointer;">
            <td class="align-middle py-2">
              <div class="d-flex flex-column">
                <span class="fw-medium">{{ patient.full_name }}</span>
                <small class="text-muted font-mono-small">
                  ID: {{ truncateId(patient.patient_id) }}
                </small>
              </div>
            </td>
            <td class="align-middle py-2 text-end">
              <span class="badge bg-light text-dark rounded-pill px-3">
                <i class="bi bi-folder2-open me-1"></i>
                {{ patient.session_count }}
              </span>
            </td>
            <td class="align-middle py-2 text-end">
              <button
                  class="btn btn-danger btn-sm delete-btn-circular hover-reveal"
                  @click.stop="$emit('delete-patient', patient)"
                  title="Delete patient">
                <i class="bi bi-x-lg"></i>
              </button>
            </td>
          </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
import { truncateId } from '../../utils/helpers.js';

export default {
  props: {
    patients: Array,
    selectedPatientId: String
  },
  methods: {
    truncateId
  }
}
</script>