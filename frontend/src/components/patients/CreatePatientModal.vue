<template>
  <!-- Modal -->
  <div class="modal fade" id="createPatientModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content" style="border-radius: 20px; border: none;">
        <div class="modal-header border-0 pb-0">
          <h5 class="modal-title fw-bold">Create New Patient</h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body p-4">
          <form @submit.prevent="handleSubmit">
            <div class="mb-3">
              <label class="form-label fw-semibold">First Name</label>
              <input
                  v-model="formData.first_name"
                  type="text"
                  class="form-control rounded-pill"
                  required>
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">Last Name</label>
              <input
                  v-model="formData.last_name"
                  type="text"
                  class="form-control rounded-pill"
                  required>
            </div>

            <div class="mb-3">
              <label class="form-label fw-semibold">Date of Birth</label>
              <input
                  v-model="formData.date_of_birth"
                  type="date"
                  class="form-control rounded-pill"
                  required>
            </div>

            <div v-if="error" class="alert alert-danger rounded-3" role="alert">
              {{ error }}
            </div>

            <div class="d-flex gap-2 justify-content-end mt-4">
              <button type="button" class="btn btn-secondary rounded-pill px-4" data-bs-dismiss="modal">
                Cancel
              </button>
              <button type="submit" class="btn btn-primary rounded-pill px-4" :disabled="loading">
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

export default {
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
        const response = await fetch('http://localhost:8000/api/patients/create', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(this.formData)
        });

        if (!response.ok) {
          throw new Error('Failed to create patient');
        }

        const data = await response.json();
        this.$emit('patient-created', data);
        this.hide();
        this.resetForm();

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