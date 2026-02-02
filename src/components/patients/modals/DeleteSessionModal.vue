<template>
  <div class="modal fade" id="deleteSessionModal" tabindex="-1" aria-hidden="true">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content rounded-card">
        <div class="modal-header border-0">
          <h5 class="modal-title fw-bold text-danger">
            <i class="bi bi-exclamation-triangle-fill me-2"></i>
            Delete Session
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
        </div>

        <div class="modal-body" style="position: relative;">
          <LoadingSpinner :show="deleting" message="Deleting session..." />

          <div :class="{ 'opacity-50': deleting }">
            <p class="mb-3 text-dark">Are you sure you want to delete this session?</p>

            <div v-if="session" class="modal-info-card">
              <div class="d-flex align-items-center gap-3 mb-2">
                <div class="bg-info bg-opacity-10 rounded-circle p-3">
                  <i class="bi bi-clipboard-data-fill fs-4 text-info"></i>
                </div>
                <div>
                  <h6 class="fw-bold mb-1">{{ session.session_name || 'Unnamed Session' }}</h6>
                  <small class="text-muted">
                    {{ formatDate(session.started_at) }}
                  </small>
                </div>
              </div>
              <div class="mt-2 pt-2 border-top">
                <small class="text-muted">
                  <i class="bi bi-activity me-1"></i>
                  {{ session.reading_count }} reading{{ session.reading_count !== 1 ? 's' : '' }}
                </small>
              </div>
            </div>

            <div class="modal-danger-warning">
              <div class="d-flex gap-2">
                <i class="bi bi-exclamation-triangle-fill text-warning"></i>
                <div>
                  <strong class="d-block mb-1">This action cannot be undone</strong>
                  <small class="text-dark">
                    This will permanently delete the session and all its recorded data.
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
                {{ deleting ? 'Deleting...' : 'Delete Session' }}
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
import { formatDate } from '../../../utils/helpers.js';
import LoadingSpinner from '../../../utils/Loading.vue';

export default {
  components: {
    LoadingSpinner
  },
  data() {
    return {
      session: null,
      modalInstance: null,
      deleting: false
    }
  },
  mounted() {
    this.modalInstance = new Modal(document.getElementById('deleteSessionModal'));
  },
  methods: {
    formatDate,

    show(session) {
      this.session = session;
      this.modalInstance.show();
    },

    hide() {
      this.modalInstance.hide();
    },

    confirmDelete() {
      this.$emit('confirm-delete', this.session.session_id);
      this.hide();
    }
  }
}
</script>