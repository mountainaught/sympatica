// Sidebar.vue
<template>
  <div class="card shadow-lg border-0 sidebar-width rounded-card">
    <div class="card-body p-4">
      <h4 class="fw-bold mb-4 text-center">Menu</h4>
      <div class="d-grid gap-3">
        <router-link :to="{ path: '/home', query: $route.query }" custom v-slot="{ navigate, isActive }">
          <button
              class="btn btn-lg rounded-pill shadow-sm"
              :class="isActive ? 'btn-primary' : 'btn-light'"
              @click="navigate">
            Home
          </button>
        </router-link>

        <router-link :to="{ path: '/patients', query: $route.query }" custom v-slot="{ navigate, isActive }">
          <button
              class="btn btn-lg rounded-pill shadow-sm"
              :class="isActive ? 'btn-primary' : 'btn-light'"
              @click="navigate">
            Patients
          </button>
        </router-link>
      </div>
    </div>
  </div>
</template>