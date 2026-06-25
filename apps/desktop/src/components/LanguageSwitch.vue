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
    <span class="language-switch-label">{{ nextLocaleLabel }}</span>
  </button>
</template>

<script setup lang="ts">
import { Languages } from '@lucide/vue'
import { computed } from 'vue'

import { currentLocale, toggleLocale } from '@/i18n'

const nextLocaleLabel = computed(() => (currentLocale.value === 'zh-CN' ? 'EN' : '中文'))
</script>

<style scoped>
.language-switch {
  min-width: 78px;
  height: 48px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border: 1px solid var(--panel-border);
  border-radius: 999px;
  background: #ffffff;
  color: #52525b;
  padding: 0 14px;
  font-size: 13px;
  font-weight: 700;
  line-height: 1;
  cursor: pointer;
  transition:
    background-color 0.16s ease,
    border-color 0.16s ease,
    color 0.16s ease;
}

.language-switch:hover {
  border-color: #d4d4d8;
  background: #f7f7f8;
  color: #18181b;
}

.language-switch-icon {
  width: 17px;
  height: 17px;
  flex: 0 0 auto;
  opacity: 0.72;
}

.language-switch:hover .language-switch-icon {
  opacity: 1;
}

.language-switch-label {
  min-width: 24px;
  text-align: left;
}
</style>
