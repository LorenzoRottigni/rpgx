import type { App } from 'vue'
import RPGXEngine from './components/Engine.vue'

export default {
  install(app: App) {
    app.config.globalProperties.$RPGX = () => {
      console.log('RPGX-vue installed');
      app.component('RPGXEngine', RPGXEngine);
    };
  }
};