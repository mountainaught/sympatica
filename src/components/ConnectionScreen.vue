<template>
  <div class="h-screen flex items-center justify-center p-8" style="background: var(--gradient-bg-connection)">
    <div class="card shadow-2xl w-full max-w-2xl">
      <!-- Header -->
      <div class="border-b border-default p-8 text-right">
        <i class="bi bi-heart-pulse-fill text-6xl text-brand animate-heartbeat"></i>
        <h1 class="text-4xl font-bold text-primary mt-4 tracking-tight">SYMPATICA</h1>
        <p class="text-muted mt-2 uppercase tracking-wider text-sm font-semibold">Empatica E4 Monitoring System</p>
      </div>

      <!-- Device Selection -->
      <div class="p-8">
        <div class="mb-6">
          <h2 class="text-xl font-semibold text-primary mb-2">Connect Your Device</h2>
          <p class="text-secondary text-sm">Select your Empatica E4 from the list below</p>
        </div>

        <!-- Scanning State -->
        <div v-if="isScanning && devices.length === 0" class="py-12 text-center">
          <div class="inline-block animate-spin h-12 w-12 border-4 border-default mb-4" style="border-top-color: var(--color-brand)"></div>
          <p class="text-secondary font-medium">Scanning for devices...</p>
          <p class="text-muted text-sm mt-2">Make sure your E4 is powered on and nearby</p>
        </div>

        <!-- Device List -->
        <div v-else-if="devices.length > 0" class="space-y-2">
          <button
              v-for="device in devices"
              :key="device.id"
              @click="selectDevice(device)"
              :disabled="isConnecting"
              class="card card-hover w-full flex items-center justify-between p-4 border-2 transition-all duration-150 disabled:opacity-50 disabled:cursor-not-allowed"
          >
            <div class="flex items-center gap-4">
              <i class="bi bi-bluetooth text-2xl text-brand"></i>
              <div class="text-left">
                <p class="font-semibold text-primary">{{ device.name }}</p>
                <p class="text-xs text-muted font-mono">{{ device.id }}</p>
              </div>
            </div>
            <i v-if="!isConnecting" class="bi bi-chevron-right text-muted"></i>
            <div v-else class="animate-spin h-5 w-5 border-2 border-default" style="border-top-color: var(--color-brand)"></div>
          </button>
        </div>

        <!-- No Devices Found -->
        <div v-else class="py-12 text-center border-2 border-dashed border-default bg-app">
          <i class="bi bi-exclamation-circle text-5xl text-muted mb-4"></i>
          <p class="text-primary font-semibold mb-2">No devices found</p>
          <p class="text-secondary text-sm mb-4">Please check your device is powered on</p>
          <button
              @click="startScan"
              :disabled="isScanning"
              class="btn btn-primary"
          >
            <i class="bi bi-arrow-clockwise mr-2"></i>RESCAN
          </button>
        </div>

        <!-- Error State -->
        <div v-if="error" class="mt-4 p-4 border-l-4" style="background-color: #fef2f2; border-color: var(--color-danger);">
          <div class="flex items-start gap-3">
            <i class="bi bi-exclamation-triangle-fill text-danger text-lg"></i>
            <div class="flex-1">
              <p class="text-primary font-semibold text-sm">Connection Failed</p>
              <p class="text-secondary text-sm mt-1">{{ error }}</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="border-t border-default p-6 bg-app">
        <div class="flex items-center justify-between text-xs text-muted">
          <span class="font-medium">Requires Bluetooth connectivity</span>
          <span class="font-mono">v1.0.0</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/core'

export default {
  emits: ['device-connected'],
  data() {
    return {
      devices: [],
      isScanning: false,
      isConnecting: false,
      error: null,
      scanInterval: null
    }
  },
  mounted() {
    this.startScan()
  },
  beforeUnmount() {
    if (this.scanInterval) {
      clearInterval(this.scanInterval)
    }
  },
  methods: {
    async startScan() {
      this.isScanning = true
      this.error = null

      try {
        const foundDevices = await invoke('scan_for_devices')
        this.devices = foundDevices
      } catch (err) {
        this.error = err.toString()
        this.devices = []
      } finally {
        this.isScanning = false
      }

      if (this.scanInterval) clearInterval(this.scanInterval)
      this.scanInterval = setInterval(async () => {
        try {
          const foundDevices = await invoke('scan_for_devices')
          this.devices = foundDevices
        } catch (err) {
          console.error('Scan error:', err)
        }
      }, 5000)
    },

    async selectDevice(device) {
      this.isConnecting = true
      this.error = null

      try {
        const result = await invoke('connect_device', { id: device.id })

        this.$emit('device-connected', {
          name: device.name,
          id: device.id
        })
      } catch (err) {
        this.error = err.toString()
      } finally {
        this.isConnecting = false
      }
    }
  }
}
</script>