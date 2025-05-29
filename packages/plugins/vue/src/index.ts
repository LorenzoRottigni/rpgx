import type { App } from 'vue'
import RPGXEngine from './components/Engine.vue'

export default {
  install(app: App) {
    app.component('RPGXEngine', RPGXEngine);
  }
};