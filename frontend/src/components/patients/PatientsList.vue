<template>
  <div class="card shadow-lg border-0" style="flex: 1; border-radius: 20px; overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column h-100">
      <div class="d-flex justify-content-between align-items-center mb-3 pb-3">
        <h3 class="fw-bold mb-0">Patients</h3>
        <button class="btn btn-primary btn-sm rounded-pill px-3" @click="$emit('create-patient')">
          + New Patient
        </button>
      </div>

      <div class="flex-grow-1 overflow-auto patient-list">
        <table class="table table-hover mb-0">
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
              class="patient-row"
              :class="{ 'table-active': selectedPatientId === patient.patient_id }"
              @click="$emit('select-patient', patient)"
              style="cursor: pointer;">
            <td class="align-middle py-2">
              <div class="d-flex flex-column">
                <span class="fw-medium">{{ patient.full_name }}</span>
                <small class="text-muted font-monospace" style="font-size: 0.75rem;">
                  ID: {{ patient.patient_id.slice(0, 8) }}
                </small>
              </div>
            </td>
            <td class="align-middle py-2 text-end">
                <span class="badge bg-light text-dark rounded-pill px-2">
                  {{ patient.session_count }}
                </span>
            </td>
            <td class="align-middle py-2 text-end">
              <button
                  class="btn btn-danger btn-sm rounded-circle delete-btn"
                  @click.stop="$emit('delete-patient', patient)"
                  title="Delete patient">
                ✖
              </button>
            </td>
          </tr>
          </tbody>
        </table>

        <div v-if="patients.length === 0" class="text-center text-muted py-5">
          <p>No patients found</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    patients: Array,
    selectedPatientId: String
  }
}
</script>

<style scoped>
.patient-list {
  overflow-y: auto;
  max-height: 100%;
  /* Hide scrollbar but keep scrolling */
  scrollbar-width: none; /* Firefox */
  -ms-overflow-style: none; /* IE/Edge */
}

.patient-list::-webkit-scrollbar {
  display: none; /* Chrome/Safari */
}

.table thead th {
  border-top: none;
  border-bottom: 2px solid #dee2e6;
  letter-spacing: 0.5px;
}

.patient-row {
  transition: background-color 0.15s ease;
}

.patient-row:hover {
  background-color: #f8f9fa;
}

.table-active {
  background-color: #e7f3ff !important;
}

.patient-row td {
  border-bottom: 1px solid #f0f0f0;
}

.font-monospace {
  font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
}
</style>