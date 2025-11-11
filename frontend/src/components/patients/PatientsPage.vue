<template>
  <div class="vh-100 bg-gradient p-3" style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);">
    <div class="d-flex gap-3 h-100">
      <Sidebar :currentPage="'patients'" @navigate="$emit('navigate', $event)" />

      <div class="flex-grow-1 d-flex gap-3 patients-container">
        <PatientsList
            :patients="patients"
            :selectedPatientId="selectedPatient?.patient_id"
            @select-patient="selectPatient"
            @create-patient="openCreatePatientModal"
            @delete-patient="deletePatient" />

        <SessionsPanel
            :patient="selectedPatient"
            :sessions="sessions" />
      </div>
    </div>

    <!-- Modal Component -->
    <CreatePatientModal
        ref="createModal"
        @patient-created="onPatientCreated" />
  </div>
</template>

<script>
import Sidebar from '../shared/Sidebar.vue';
import PatientsList from './PatientsList.vue';
import SessionsPanel from './SessionsPanel.vue';
import CreatePatientModal from './CreatePatientModal.vue';

export default {
  components: {
    Sidebar,
    PatientsList,
    SessionsPanel,
    CreatePatientModal
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
        const response = await fetch('http://localhost:8000/api/patients/');
        const data = await response.json();
        this.patients = data;

        for (let patient of this.patients) {
          const sessionsResponse = await fetch(`http://localhost:8000/api/sessions/?patient_id=${patient.patient_id}`);
          const sessionsData = await sessionsResponse.json();
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

    async deletePatient(patientId) {
      if (!confirm('Are you sure you want to delete this patient?')) {
        return;
      }

      try {
        const response = await fetch(`http://localhost:8000/api/patients/${patientId}/delete`, {
          method: 'DELETE'
        });

        if (!response.ok) {
          throw new Error('Failed to delete patient');
        }

        await this.loadPatients();

        // Clear selection if deleted patient was selected
        if (this.selectedPatient?.patient_id === patientId) {
          this.selectedPatient = null;
          this.sessions = [];
        }
      } catch (error) {
        console.error('Error deleting patient:', error);
        alert('Failed to delete patient');
      }
    },

    async loadSessions(patientId) {
      try {
        const response = await fetch(`http://localhost:8000/api/sessions/?patient_id=${patientId}`);
        this.sessions = await response.json();
      } catch (error) {
        console.error('Error loading sessions:', error);
      }
    },

    openCreatePatientModal() {
      this.$refs.createModal.show();
    },

    async onPatientCreated(newPatient) {
      await this.loadPatients();
    }
  }
}
</script>

<style scoped>
@media (max-width: 900px) {
  .patients-container {
    flex-direction: column !important;
  }
}
</style>