<template>
  <div v-if="!isConnected" class="h-screen">
    <ConnectionScreen @device-connected="handleConnection" />
  </div>

  <div v-else class="flex h-screen overflow-hidden">
    <Sidebar
        :current-page="currentPage"
        :device-info="deviceInfo"
        :is-streaming="isStreaming"
        @navigate="currentPage = $event"
        @reconnect="handleReconnect"
        @toggle-streaming="toggleStreaming"
    />

    <main class="flex-1 overflow-hidden">
      <component :is="currentComponent" :is-streaming="isStreaming" />
    </main>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'
import Sidebar from './components/Sidebar.vue'
import ConnectionScreen from './components/ConnectionScreen.vue'
import ReadingsPage from './components/readings/ReadingsPage.vue'
import PatientsPage from './components/patients/PatientsPage.vue'
import SettingsPage from './components/settings/SettingsPage.vue'

export default {
  components: {
    Sidebar,
    ConnectionScreen,
    ReadingsPage,
    PatientsPage,
    SettingsPage
  },
  data() {
    return {
      currentPage: 'readings',
      isConnected: false,
      isStreaming: false,
      deviceInfo: null
    }
  },
  computed: {
    currentComponent() {
      const pages = {
        readings: 'ReadingsPage',
        patients: 'PatientsPage',
        settings: 'SettingsPage'
      }
      return pages[this.currentPage]
    }
  },
  methods: {
    handleConnection(device) {
      this.deviceInfo = device
      this.isConnected = true
      console.log('Device connected:', device)
    },

    handleReconnect() {
      this.isConnected = false
      this.isStreaming = false
      this.deviceInfo = null
    },

    async toggleStreaming() {
      try {
        if (this.isStreaming) {
          console.log('Calling stop_streaming...')
          await invoke('stop_streaming')
          this.isStreaming = false
          console.log('Streaming stopped')
        } else {
          console.log('Calling start_streaming...')
          await invoke('start_streaming')
          this.isStreaming = true
          console.log('Streaming started successfully')
        }
      } catch (err) {
        console.error('Streaming toggle failed:', err)
        alert('Streaming error: ' + err)
      }
    }
  }
}
</script>