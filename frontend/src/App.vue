<template>
  <div>
    <HomePage v-if="currentPage === 'home'" @navigate="changePage" />
    <PatientsPage v-if="currentPage === 'patients'" @navigate="changePage" />
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
      currentPage: 'home'
    }
  },
  methods: {
    changePage(page) {
      this.currentPage = page;
    }
  }
}
</script>