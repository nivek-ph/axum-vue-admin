<template>
  <label :class="label ? 'grid grid-cols-[110px_minmax(0,1fr)] items-center gap-3 mb-4' : 'block mb-4'">
    <span v-if="label" class="text-right text-sm font-semibold text-zinc-600">{{ t(label) }}</span>
    <span class="min-w-0">
      <slot />
    </span>
  </label>
</template>

<script setup lang="ts">
import { t } from '@/i18n'

defineProps<{
  label?: string
  labelWidth?: string
}>()
</script>
