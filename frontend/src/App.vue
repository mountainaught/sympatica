<template>
  <div class="vh-100 page-bg p-3">
    <div class="d-flex gap-3 h-100">
      <!-- Sidebar always visible! -->
      <Sidebar :currentPage="currentPage" @navigate="changePage" />

      <!-- Main content area -->
      <div class="flex-grow-1 d-flex" style="min-width: 0;">
        <HomePage
            v-if="currentPage === 'home'"
            :activeSession="activeSession"
            @set-active-session="setActiveSession" />
        <PatientsPage
            v-if="currentPage === 'patients'"
            :activeSession="activeSession"
            @set-active-session="setActiveSession" />
      </div>
    </div>
  </div>
</template>

<script>
import Sidebar from './components/Sidebar.vue';
import HomePage from './components/home/HomePage.vue';
import PatientsPage from './components/patients/PatientsPage.vue';

export default {
  components: {
    Sidebar,
    HomePage,
    PatientsPage
  },
  data() {
    return {
      currentPage: 'home',
      activeSession: null
    }
  },
  methods: {
    changePage(page) {
      this.currentPage = page;
    },

    setActiveSession(sessionData) {
      this.activeSession = sessionData;
      console.log('Active session set:', sessionData);
    }
  }
}
</script>