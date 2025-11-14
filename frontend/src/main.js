import { createApp } from 'vue'
import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap/dist/js/bootstrap.bundle.min.js'
import 'bootstrap-icons/font/bootstrap-icons.css'
import 'uplot/dist/uPlot.min.css'
import './styles/main.css'

import App from './App.vue'
import router from './router.js'

const app = createApp(App)
app.use(router)
app.mount('#app')