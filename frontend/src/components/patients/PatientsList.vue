<template>
  <div class="card shadow-lg border-0" style="flex: 1; border-radius: 20px; overflow: hidden;">
    <div class="card-body p-4 d-flex flex-column">
      <div class="d-flex justify-content-between align-items-center mb-3">
        <h3 class="fw-bold mb-0">Patients</h3>
        <button class="btn btn-primary rounded-pill px-4" @click="$emit('create-patient')">
          ➕ New Patient
        </button>
      </div>

      <div class="flex-grow-1 overflow-auto">
        <div class="list-group list-group-flush">
          <button
              v-for="patient in patients"
              :key="patient.patient_id"
              class="list-group-item list-group-item-action border-0 rounded-3 mb-2 patient-item"
              :class="{ 'active': selectedPatientId === patient.patient_id }"
              @click="$emit('select-patient', patient)">
            <div class="d-flex justify-content-between align-items-center">
              <div>
                <h5 class="mb-1">{{ patient.full_name }}</h5>
                <small class="text-muted">{{ patient.session_count }} sessions</small>
              </div>
              <div class="d-flex align-items-center gap-2">
                <i class="bi bi-chevron-right chevron-icon"></i>
                <button
                    class="btn btn-danger btn-sm rounded-circle delete-btn"
                    @click.stop="$emit('delete-patient', patient.patient_id)"
                    title="Delete patient">
                  🗑️
                </button>
              </div>
            </div>
          </button>
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
.list-group-item.active {
  background-color: #667eea !important;
  border-color: #667eea !important;
  color: white;
}

.patient-item {
  position: relative;
}

.delete-btn {
  opacity: 0;
  transition: opacity 0.2s;
  width: 32px;
  height: 32px;
  padding: 0;
}

.patient-item:hover .delete-btn {
  opacity: 1;
}

.patient-item:hover .chevron-icon {
  opacity: 0;
}

.chevron-icon {
  transition: opacity 0.2s;
}
</style>