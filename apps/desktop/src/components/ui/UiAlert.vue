<template>
  <div :class="classes">
    <span v-if="showIcon" class="font-bold">!</span>
    <span><slot>{{ title ? t(title) : '' }}</slot></span>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { t } from '@/i18n'

const props = defineProps<{
  title?: string
  type?: 'success' | 'warning' | 'error' | 'info' | string
  showIcon?: boolean
  closable?: boolean
}>()

const classes = computed(() => [
  'flex items-center gap-2 rounded-md border px-4 py-3 text-sm font-medium',
  props.type === 'success' && 'border-emerald-200 bg-emerald-50 text-emerald-700',
  props.type === 'warning' && 'border-amber-200 bg-amber-50 text-amber-700',
  props.type === 'error' && 'border-red-200 bg-red-50 text-red-700',
  (!props.type || props.type === 'info') && 'border-stone-200 bg-stone-50 text-zinc-700'
])
</script>
