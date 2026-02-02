<template>
  <div class="modal fade" id="createSessionModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content rounded-card">
        <div class="modal-header border-0">
          <h5 class="modal-title fw-bold">
            <i class="bi bi-clipboard-plus-fill me-2 text-success"></i>
            Create New Session
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body" style="position: relative;">
          <LoadingSpinner :show="loading" message="Creating session..." />

          <form @submit.prevent="handleSubmit" :class="{ 'opacity-50': loading }">
            <div class="mb-3">
              <label class="form-label fw-semibold">
                <i class="bi bi-tag me-1"></i>
                Session Name
              </label>
              <input
                  v-model="formData.session_name"
                  type="text"
                  class="form-control rounded-pill"
                  placeholder="e.g., Morning Session"
                  :disabled="loading">
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">
                <i class="bi bi-journal-text me-1"></i>
                Notes (optional)
              </label>
              <textarea
                  v-model="formData.notes"
                  class="form-control rounded-3"
                  rows="4"
                  placeholder="Add any relevant notes about this session..."
                  :disabled="loading"></textarea>
            </div>

            <div v-if="error" class="alert alert-danger rounded-3 d-flex align-items-center" role="alert">
              <i class="bi bi-exclamation-triangle-fill me-2"></i>
              {{ error }}
            </div>

            <div class="modal-footer-actions">
              <button type="button" class="btn btn-secondary rounded-pill px-4" data-bs-dismiss="modal" :disabled="loading">
                <i class="bi bi-x-lg me-1"></i>
                Cancel
              </button>
              <button type="submit" class="btn btn-success rounded-pill px-4" :disabled="loading">
                <span v-if="loading" class="spinner-border spinner-border-sm me-2"></span>
                <i v-else class="bi bi-check-lg me-1"></i>
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
import LoadingSpinner from '../../../utils/Loading.vue';

export default {
  components: {
    LoadingSpinner
  },
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
        window.$toast.addToast({
          title: 'Session Created',
          message: `${data.session_name || 'Session'} created successfully`,
          type: 'success'
        });
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