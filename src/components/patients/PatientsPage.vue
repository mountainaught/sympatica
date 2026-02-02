// PatientsPage.vue
<template>
  <div class="flex-grow-1 d-flex gap-3 patients-container">
    <Loading :show="loading" message="Loading patients..." />

    <PatientsList
        :patients="patients"
        :selectedPatientId="selectedPatient?.patient_id"
        @select-patient="selectPatient"
        @create-patient="openCreatePatientModal"
        @delete-patient="openDeletePatientModal" />

    <SessionsPanel
        :patient="selectedPatient"
        :sessions="sessions"
        :activeSessionId="activeSession?.session_id"
        @create-session="openCreateSessionModal"
        @delete-session="openDeleteSessionModal"
        @set-active-session="setActiveSession" />

    <CreatePatientModal
        ref="createPatientModal"
        @patient-created="onPatientCreated" />

    <DeleteConfirmModal
        ref="deletePatientModal"
        @confirm-delete="deletePatient" />

    <CreateSessionModal
        ref="createSessionModal"
        @session-created="onSessionCreated" />

    <DeleteSessionModal
        ref="deleteSessionModal"
        @confirm-delete="deleteSession" />
  </div>
</template>

<script>
import PatientsList from './PatientsList.vue';
import SessionsPanel from './SessionsPanel.vue';
import CreatePatientModal from './modals/CreatePatientModal.vue';
import DeleteConfirmModal from './modals/DeleteConfirmModal.vue';
import CreateSessionModal from './modals/CreateSessionModal.vue';
import DeleteSessionModal from './modals/DeleteSessionModal.vue';
import Loading from '../../utils/Loading.vue';
import {deleteAPI, fetchAPI} from '../../utils/helpers.js';

export default {
  components: {
    PatientsList,
    SessionsPanel,
    CreatePatientModal,
    DeleteConfirmModal,
    CreateSessionModal,
    DeleteSessionModal,
    Loading
  },
  computed: {
    activeSession() {
      const sessionId = this.$route.query.session;
      if (!sessionId) return null;
      return this.sessions.find(s => s.session_id === sessionId);
    }
  },
  data() {
    return {
      patients: [],
      selectedPatient: null,
      sessions: [],
      loading: false
    }
  },
  mounted() {
    this.loadPatients();
  },
  methods: {
    async loadPatients() {
      this.loading = true;
      try {
        this.patients = await fetchAPI('/patients/');

        for (let patient of this.patients) {
          const sessionsData = await fetchAPI(`/sessions/?patient_id=${patient.patient_id}`);
          patient.session_count = sessionsData.length;
        }
      } catch (error) {
        console.error('Error loading patients:', error);
      } finally {
        this.loading = false;
      }
    },

    async selectPatient(patient) {
      this.selectedPatient = patient;
      await this.loadSessions(patient.patient_id);
    },

    async loadSessions(patientId) {
      try {
        this.sessions = await fetchAPI(`/sessions/?patient_id=${patientId}`);
      } catch (error) {
        console.error('Error loading sessions:', error);
      }
    },

    openCreatePatientModal() {
      this.$refs.createPatientModal.show();
    },

    openDeletePatientModal(patient) {
      this.$refs.deletePatientModal.show(patient);
    },

    async deletePatient(patientId) {
      try {
        await deleteAPI(`/patients/${patientId}/delete`);
        await this.loadPatients();

        if (this.selectedPatient?.patient_id === patientId) {
          this.selectedPatient = null;
          this.sessions = [];
        }

        window.$toast.addToast({
          title: 'Patient Deleted',
          message: 'Patient record has been removed',
          type: 'success'
        });
      } catch (error) {
        console.error('Error deleting patient:', error);
        window.$toast.addToast({
          title: 'Delete Failed',
          message: 'Failed to delete patient',
          type: 'error'
        });
      }
    },
    async onPatientCreated() {
      await this.loadPatients();
    },

    openCreateSessionModal() {
      if (!this.selectedPatient) return;
      this.$refs.createSessionModal.show(this.selectedPatient.patient_id);
    },

    openDeleteSessionModal(session) {
      this.$refs.deleteSessionModal.show(session);
    },

    async deleteSession(sessionId) {
      try {
        await deleteAPI(`/sessions/${sessionId}/delete`);

        if (this.activeSession?.session_id === sessionId) {
          this.$emit('set-active-session', null);
        }

        await this.loadSessions(this.selectedPatient.patient_id);
        await this.loadPatients();

        window.$toast.addToast({
          title: 'Session Deleted',
          message: 'Session has been removed',
          type: 'success'
        });
      } catch (error) {
        console.error('Error deleting session:', error);
        window.$toast.addToast({
          title: 'Delete Failed',
          message: 'Failed to delete session',
          type: 'error'
        });
      }
    },

    async onSessionCreated() {
      await this.loadSessions(this.selectedPatient.patient_id);
      await this.loadPatients();
    },

    setActiveSession(sessionId) {
      this.$router.push({ query: { session: sessionId } });
    }
  }
}
</script>