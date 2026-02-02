<template>
  <div class="modal fade" id="createPatientModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content rounded-card">
        <div class="modal-header border-0">
          <h5 class="modal-title fw-bold">
            <i class="bi bi-person-plus-fill me-2 text-primary"></i>
            Create New Patient
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body" style="position: relative;">
          <LoadingSpinner :show="loading" message="Creating patient..." />

          <form @submit.prevent="handleSubmit" :class="{ 'opacity-50': loading }">
            <div class="mb-3">
              <label class="form-label fw-semibold">
                <i class="bi bi-person me-1"></i>
                First Name
              </label>
              <input
                  v-model="formData.first_name"
                  type="text"
                  class="form-control rounded-pill"
                  placeholder="Enter first name"
                  required
                  :disabled="loading">
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">
                <i class="bi bi-person me-1"></i>
                Last Name
              </label>
              <input
                  v-model="formData.last_name"
                  type="text"
                  class="form-control rounded-pill"
                  placeholder="Enter last name"
                  required
                  :disabled="loading">
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">
                <i class="bi bi-calendar-event me-1"></i>
                Date of Birth
              </label>
              <input
                  v-model="formData.date_of_birth"
                  type="date"
                  class="form-control rounded-pill"
                  required
                  :disabled="loading">
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
              <button type="submit" class="btn btn-primary rounded-pill px-4" :disabled="loading">
                <span v-if="loading" class="spinner-border spinner-border-sm me-2"></span>
                <i v-else class="bi bi-check-lg me-1"></i>
                {{ loading ? 'Creating...' : 'Create Patient' }}
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
      formData: {
        first_name: '',
        last_name: '',
        date_of_birth: ''
      },
      loading: false,
      error: null,
      modalInstance: null
    }
  },
  mounted() {
    this.modalInstance = new Modal(document.getElementById('createPatientModal'));
  },
  methods: {
    show() {
      this.modalInstance.show();
      this.resetForm();
    },

    hide() {
      this.modalInstance.hide();
    },

    resetForm() {
      this.formData = {
        first_name: '',
        last_name: '',
        date_of_birth: ''
      };
      this.error = null;
    },

    async handleSubmit() {
      this.loading = true;
      this.error = null;

      try {
        const data = await postAPI('/patients/create', this.formData);
        this.$emit('patient-created', data);
        this.hide();
        this.resetForm();
        window.$toast.addToast({
          title: 'Patient Created',
          message: `${data.full_name} has been added successfully`,
          type: 'success'
        });
      } catch (error) {
        console.error('Error creating patient:', error);
        this.error = 'Failed to create patient. Please try again.';
      } finally {
        this.loading = false;
      }
    }
  }
}
</script>