import { createApp } from 'vue';
import App from './App.vue';
import RPGXVue from '@rpgx/vue';

const app = createApp(App);
app.use(RPGXVue);
app.mount('#app');
