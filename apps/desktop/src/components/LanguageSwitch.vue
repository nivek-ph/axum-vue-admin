<template>
  <button
    class="language-switch"
    type="button"
    :aria-label="$t('Language')"
    :aria-pressed="currentLocale === 'en-US'"
    :title="$t('Language')"
    @click="toggleLocale"
  >
    <Languages class="language-switch-icon" aria-hidden="true" :stroke-width="2.1" />
  </button>
</template>

<script setup lang="ts">
import { Languages } from '@lucide/vue'

import { currentLocale, toggleLocale } from '@/i18n'
</script>

<style scoped>
.language-switch {
  width: 38px;
  height: 38px;
  display: grid;
  place-items: center;
  border: 1px solid var(--panel-border);
  border-radius: 12px;
  background: #ffffff;
  color: #27272a;
  transition:
    background-color 0.18s ease,
    border-color 0.18s ease,
    color 0.18s ease;
}

.language-switch:hover {
  border-color: #d4d4d8;
  background: #fafafa;
  color: #18181b;
}

.language-switch-icon {
  width: 19px;
  height: 19px;
}
</style>
