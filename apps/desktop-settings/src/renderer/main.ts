import { createApp } from 'vue'
import { createPinia } from 'pinia'
import '@unocss/reset/tailwind.css'
import 'uno.css'
import './styles.css'
import App from './App.vue'

const app = createApp(App)

app.use(createPinia())
app.mount('#app')
