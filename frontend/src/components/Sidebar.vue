// components/Sidebar.vue
<template>
  <div class="card shadow-lg border-0 sidebar-width rounded-card">
    <div class="card-body p-4 d-flex flex-column h-100">
      <!-- Logo/Brand -->
      <div class="text-center mb-4 pb-4 border-bottom">
        <div class="sidebar-logo mb-2">
          <i class="bi bi-heart-pulse-fill"></i>
        </div>
        <h4 class="fw-bold mb-1">Sympatica</h4>
        <small class="text-muted">Biometric Monitoring</small>
      </div>

      <!-- Navigation -->
      <nav class="d-grid gap-2 flex-grow-1">
        <router-link :to="{ path: '/home', query: $route.query }" custom v-slot="{ navigate, isActive }">
          <button
              class="btn btn-sidebar d-flex align-items-center gap-3"
              :class="isActive ? 'btn-sidebar-active' : 'btn-sidebar-inactive'"
              @click="navigate">
            <i class="bi bi-house-door-fill fs-5"></i>
            <span class="fw-semibold">Home</span>
          </button>
        </router-link>

        <router-link :to="{ path: '/patients', query: $route.query }" custom v-slot="{ navigate, isActive }">
          <button
              class="btn btn-sidebar d-flex align-items-center gap-3"
              :class="isActive ? 'btn-sidebar-active' : 'btn-sidebar-inactive'"
              @click="navigate">
            <i class="bi bi-people-fill fs-5"></i>
            <span class="fw-semibold">Patients</span>
          </button>
        </router-link>
      </nav>
    </div>
  </div>
</template>