<template>
  <div class="modal fade" id="deleteConfirmModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content rounded-card">
        <div class="modal-header border-0">
          <h5 class="modal-title fw-bold text-danger">
            <i class="bi bi-exclamation-triangle-fill me-2"></i>
            Delete Patient
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body" style="position: relative;">
          <LoadingSpinner :show="deleting" message="Deleting patient..." />

          <div :class="{ 'opacity-50': deleting }">
            <p class="mb-3 text-dark">Are you sure you want to delete this patient?</p>

            <div v-if="patient" class="modal-info-card">
              <div class="d-flex align-items-center gap-3 mb-2">
                <div class="bg-primary bg-opacity-10 rounded-circle p-3">
                  <i class="bi bi-person-fill fs-4 text-primary"></i>
                </div>
                <div>
                  <h6 class="fw-bold mb-1">{{ patient.full_name }}</h6>
                  <small class="text-muted font-mono-small">
                    ID: {{ truncateId(patient.patient_id) }}...
                  </small>
                </div>
              </div>
              <div class="mt-2 pt-2 border-top">
                <small class="text-muted">
                  <i class="bi bi-folder2-open me-1"></i>
                  {{ patient.session_count }} session{{ patient.session_count !== 1 ? 's' : '' }}
                </small>
              </div>
            </div>

            <div class="modal-danger-warning">
              <div class="d-flex gap-2">
                <i class="bi bi-exclamation-triangle-fill text-warning"></i>
                <div>
                  <strong class="d-block mb-1">This action cannot be undone</strong>
                  <small class="text-dark">
                    This will permanently delete the patient and all associated sessions and readings.
                  </small>
                </div>
              </div>
            </div>

            <div class="modal-footer-actions">
              <button type="button" class="btn btn-secondary rounded-pill px-4" data-bs-dismiss="modal" :disabled="deleting">
                <i class="bi bi-x-lg me-1"></i>
                Cancel
              </button>
              <button type="button" class="btn btn-danger rounded-pill px-4" @click="confirmDelete" :disabled="deleting">
                <span v-if="deleting" class="spinner-border spinner-border-sm me-2"></span>
                <i v-else class="bi bi-trash-fill me-1"></i>
                {{ deleting ? 'Deleting...' : 'Delete Patient' }}
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { Modal } from 'bootstrap';
import { truncateId } from '../../../utils/helpers.js';
import LoadingSpinner from '../../../utils/Loading.vue';

export default {
  components: {
    LoadingSpinner
  },
  data() {
    return {
      patient: null,
      modalInstance: null,
      deleting: false
    }
  },
  mounted() {
    this.modalInstance = new Modal(document.getElementById('deleteConfirmModal'));
  },
  methods: {
    truncateId,

    show(patient) {
      this.patient = patient;
      this.modalInstance.show();
    },

    hide() {
      this.modalInstance.hide();
    },

    confirmDelete() {
      this.$emit('confirm-delete', this.patient.patient_id);
      this.hide();
    }
  }
}
</script>