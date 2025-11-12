<template>
  <div class="flex-grow-1 d-flex gap-3 patients-container">
    <PatientsList
        :patients="patients"
        :selectedPatientId="selectedPatient?.patient_id"
        @select-patient="selectPatient"
        @create-patient="openCreatePatientModal"
        @delete-patient="openDeletePatientModal" />

    <SessionsPanel
        :patient="selectedPatient"
        :sessions="sessions"
        :activeSessionId="activeSession?.id"
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
import CreatePatientModal from '../modals/CreatePatientModal.vue';
import DeleteConfirmModal from '../modals/DeleteConfirmModal.vue';
import CreateSessionModal from '../modals/CreateSessionModal.vue';
import DeleteSessionModal from '../modals/DeleteSessionModal.vue';
import { fetchAPI, deleteAPI } from '../../utils/helpers.js';

export default {
  components: {
    PatientsList,
    SessionsPanel,
    CreatePatientModal,
    DeleteConfirmModal,
    CreateSessionModal,
    DeleteSessionModal
  },
  props: {
    activeSession: Object
  },
  data() {
    return {
      patients: [],
      selectedPatient: null,
      sessions: []
    }
  },
  mounted() {
    this.loadPatients();
  },
  methods: {
    async loadPatients() {
      try {
        const data = await fetchAPI('/patients/');
        this.patients = data;

        for (let patient of this.patients) {
          const sessionsData = await fetchAPI(`/sessions/?patient_id=${patient.patient_id}`);
          patient.session_count = sessionsData.length;
        }
      } catch (error) {
        console.error('Error loading patients:', error);
      }
    },

    async selectPatient(patient) {
      this.selectedPatient = patient;
      await this.loadSessions(patient.patient_id);
    },

    async loadSessions(patientId) {
      try {
        const data = await fetchAPI(`/sessions/?patient_id=${patientId}`);
        this.sessions = data;
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
      } catch (error) {
        console.error('Error deleting patient:', error);
        alert('Failed to delete patient');
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

        if (this.activeSession?.id === sessionId) {
          this.$emit('set-active-session', null);
        }

        await this.loadSessions(this.selectedPatient.patient_id);
        await this.loadPatients();
      } catch (error) {
        console.error('Error deleting session:', error);
        alert('Failed to delete session');
      }
    },

    async onSessionCreated() {
      await this.loadSessions(this.selectedPatient.patient_id);
      await this.loadPatients();
    },

    setActiveSession(sessionId) {
      const session = this.sessions.find(s => s.id === sessionId);
      if (session) {
        const sessionData = {
          ...session,
          patient_name: this.selectedPatient.full_name,
          patient_id: this.selectedPatient.patient_id
        };
        this.$emit('set-active-session', sessionData);
      }
    }
  }
}
</script>