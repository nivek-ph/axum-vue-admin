import { createPinia, setActivePinia } from 'pinia';
import ui from '@nuxt/ui/vue-plugin';
import { createApp } from 'vue';

import App from './App.vue';
import { createAppRouter } from './router';
import { bootstrapAuthSession } from './stores/bootstrapAuth';
import './styles.css';
import { ElementCompat } from './ui/elementCompat';

async function start() {
  const app = createApp(App);
  const pinia = createPinia();

  setActivePinia(pinia);
  await bootstrapAuthSession();

  const router = createAppRouter();

  app.use(pinia);
  app.use(router);
  app.use(ui);
  app.use(ElementCompat);
  app.mount('#app');
}

void start();
