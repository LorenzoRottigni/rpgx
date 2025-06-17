import { createApp } from 'vue';
import App from './App.vue';
// @ts-expect-error no types
import RPGXVue from '@rpgx/vue';

const app = createApp(App);
app.use(RPGXVue);
app.mount('#app');
