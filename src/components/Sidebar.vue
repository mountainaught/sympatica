<template>
  <div class="w-56 bg-surface border-r border-default flex flex-col h-screen">
    <!-- Logo/Header -->
    <div class="p-6 border-b border-default">
      <div class="flex items-center justify-center">
        <i class="bi bi-activity text-4xl text-brand"></i>
      </div>
      <h4 class="text-center font-bold text-primary mt-3 tracking-tight text-sm">SYMPATICA</h4>
      <p class="text-center text-xs text-muted mt-1 uppercase tracking-wider">E4 Monitor</p>
    </div>

    <!-- Start/Stop Controls -->
    <div class="p-4 border-b border-default bg-app">
      <button
          @click="$emit('toggle-streaming')"
          :disabled="!deviceInfo"
          :class="[
          'btn w-full',
          isStreaming ? 'btn-secondary' : 'btn-primary'
        ]"
      >
        <i :class="['bi', isStreaming ? 'bi-stop-fill' : 'bi-play-fill', 'mr-2']"></i>
        {{ isStreaming ? 'STOP' : 'START' }}
      </button>
    </div>

    <!-- Navigation -->
    <nav class="flex-1 p-4 space-y-1">
      <button
          v-for="item in navItems"
          :key="item.id"
          @click="$emit('navigate', item.id)"
          :class="[
          'w-full flex items-center gap-3 px-3 py-2 transition-colors duration-100 text-left text-sm',
          currentPage === item.id
            ? 'bg-brand-light text-brand border-l-2 border-brand font-semibold'
            : 'text-secondary hover:bg-app border-l-2 border-transparent'
        ]"
      >
        <i :class="['bi', item.icon, 'text-base w-4']"></i>
        <span class="uppercase tracking-wide">{{ item.label }}</span>
      </button>
    </nav>

    <!-- Device Status Footer -->
    <div class="border-t border-default bg-app">
      <!-- Connected State -->
      <div v-if="deviceInfo" class="p-4">
        <div class="flex items-center gap-2 mb-2">
          <div class="w-2 h-2 status-connected" style="background-color: currentColor;"></div>
          <span class="text-xs font-semibold text-primary uppercase tracking-wide">Connected</span>
        </div>
        <p class="text-xs text-secondary truncate">{{ deviceInfo.name }}</p>
        <p class="text-xs text-muted font-mono mt-1">{{ deviceInfo.id }}</p>
      </div>

      <!-- Disconnected State -->
      <button
          v-else
          @click="$emit('reconnect')"
          class="w-full p-4 hover:bg-surface transition-colors text-left"
      >
        <div class="flex items-center gap-2 mb-2">
          <div class="w-2 h-2 status-disconnected" style="background-color: currentColor;"></div>
          <span class="text-xs font-semibold text-muted uppercase tracking-wide">Disconnected</span>
        </div>
        <p class="text-xs text-secondary">Click to reconnect</p>
      </button>

      <!-- App Version -->
      <div class="px-4 py-3 border-t border-default">
        <div class="flex justify-between items-center text-xs text-muted">
          <span class="font-mono">v1.0.0</span>
          <span>FOSS</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  props: {
    currentPage: {
      type: String,
      default: 'readings'
    },
    deviceInfo: {
      type: Object,
      default: null
    },
    isStreaming: {
      type: Boolean,
      default: false
    }
  },
  emits: ['navigate', 'reconnect', 'toggle-streaming'],
  data() {
    return {
      navItems: [
        { id: 'readings', label: 'Readings', icon: 'bi-activity' },
        { id: 'patients', label: 'Patients', icon: 'bi-people-fill' },
        { id: 'settings', label: 'Settings', icon: 'bi-gear-fill' }
      ]
    }
  }
}
</script>