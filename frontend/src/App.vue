<template>
  <div>
    <HomePage
        v-if="currentPage === 'home'"
        :activeSession="activeSession"
        @navigate="changePage" />
    <PatientsPage
        v-if="currentPage === 'patients'"
        :activeSession="activeSession"
        @navigate="changePage"
        @set-active-session="setActiveSession" />
  </div>
</template>

<script>
import HomePage from './components/home/HomePage.vue';
import PatientsPage from './components/patients/PatientsPage.vue';

export default {
  components: {
    HomePage,
    PatientsPage
  },
  data() {
    return {
      currentPage: 'home',
      activeSession: null  // now stores full session object
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