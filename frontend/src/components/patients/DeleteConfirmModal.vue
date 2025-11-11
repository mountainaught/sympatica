<template>
  <div class="modal fade" id="deleteConfirmModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content" style="border-radius: 20px; border: none;">
        <div class="modal-header border-0 pb-0">
          <h5 class="modal-title fw-bold text-danger">Delete Patient</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body p-4">
          <p class="mb-3">Are you sure you want to delete this patient?</p>

          <div v-if="patient" class="card bg-light border-0 rounded-3 p-3 mb-3">
            <h6 class="fw-bold mb-1">{{ patient.full_name }}</h6>
            <small class="text-muted">
              ID: {{ patient.patient_id.slice(0, 8) }}... • {{ patient.session_count }} sessions
            </small>
          </div>

          <p class="text-danger small mb-0">
            This will permanently delete the patient and all associated sessions.
          </p>

          <div class="d-flex gap-2 justify-content-end mt-4">
            <button type="button" class="btn btn-secondary rounded-pill px-4" data-bs-dismiss="modal">
              Cancel
            </button>
            <button type="button" class="btn btn-danger rounded-pill px-4" @click="confirmDelete">
              Delete Patient
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { Modal } from 'bootstrap';

export default {
  data() {
    return {
      patient: null,
      modalInstance: null
    }
  },
  mounted() {
    this.modalInstance = new Modal(document.getElementById('deleteConfirmModal'));
  },
  methods: {
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