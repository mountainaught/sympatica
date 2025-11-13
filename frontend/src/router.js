// src/router.js
import { createRouter, createWebHistory } from 'vue-router';
import HomePage from './components/home/HomePage.vue';
import PatientsPage from './components/patients/PatientsPage.vue';

const routes = [
    {
        path: '/',
        redirect: '/home'
    },
    {
        path: '/home',
        name: 'Home',
        component: HomePage
    },
    {
        path: '/patients',
        name: 'Patients',
        component: PatientsPage
    }
];

const router = createRouter({
    history: createWebHistory(),
    routes
});

export default router;