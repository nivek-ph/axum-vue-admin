import { createPinia, setActivePinia } from 'pinia';
import { createApp } from 'vue';
import { VueQueryPlugin } from '@tanstack/vue-query';

import App from './App.vue';
import { createAppRouter } from './router';
import { bootstrapAuthSession } from './stores/bootstrapAuth';
import './styles.css';
import { UiComponents } from './components/ui';
import { I18nPlugin } from './i18n';
import { queryClient } from './lib/query';

async function start() {
  const app = createApp(App);
  const pinia = createPinia();

  setActivePinia(pinia);
  await bootstrapAuthSession();

  const router = createAppRouter();

  app.use(pinia);
  app.use(router);
  app.use(UiComponents);
  app.use(I18nPlugin);
  app.use(VueQueryPlugin, { queryClient });
  app.mount('#app');
}

void start();
