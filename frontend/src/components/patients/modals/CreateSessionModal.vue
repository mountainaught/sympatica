// CreateSessionModal.vue
<template>
  <div class="modal fade" id="createSessionModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content rounded-card">
        <div class="modal-header border-0 pb-0">
          <h5 class="modal-title fw-bold">Create New Session</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body p-4">
          <form @submit.prevent="handleSubmit">
            <div class="mb-3">
              <label class="form-label fw-semibold">Session Name</label>
              <input
                  v-model="formData.session_name"
                  type="text"
                  class="form-control rounded-pill"
                  placeholder="e.g., Morning Session">
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">Notes (optional)</label>
              <textarea
                  v-model="formData.notes"
                  class="form-control rounded-3"
                  rows="3"
                  placeholder="Any additional notes..."></textarea>
            </div>

            <div v-if="error" class="alert alert-danger rounded-3" role="alert">
              {{ error }}
            </div>

            <div class="d-flex gap-2 justify-content-end mt-4">
              <button type="button" class="btn btn-secondary rounded-pill px-4" data-bs-dismiss="modal">
                Cancel
              </button>
              <button type="submit" class="btn btn-success rounded-pill px-4" :disabled="loading">
                {{ loading ? 'Creating...' : 'Create Session' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { Modal } from 'bootstrap';
import { postAPI } from '../../../utils/helpers.js';

export default {
  data() {
    return {
      patientId: null,
      formData: {
        session_name: '',
        notes: ''
      },
      loading: false,
      error: null,
      modalInstance: null
    }
  },
  mounted() {
    this.modalInstance = new Modal(document.getElementById('createSessionModal'));
  },
  methods: {
    show(patientId) {
      this.patientId = patientId;
      this.modalInstance.show();
      this.resetForm();
    },

    hide() {
      this.modalInstance.hide();
    },

    resetForm() {
      this.formData = {
        session_name: '',
        notes: ''
      };
      this.error = null;
    },

    async handleSubmit() {
      this.loading = true;
      this.error = null;

      try {
        const data = await postAPI('/sessions/create', {
          patient_id: this.patientId,
          session_name: this.formData.session_name,
          notes: this.formData.notes
        });
        this.$emit('session-created', data);
        this.hide();
        this.resetForm();
      } catch (error) {
        console.error('Error creating session:', error);
        this.error = 'Failed to create session. Please try again.';
      } finally {
        this.loading = false;
      }
    }
  }
}
</script>