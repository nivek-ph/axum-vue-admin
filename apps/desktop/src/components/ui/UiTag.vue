<template>
  <span :class="classes">
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  type?: 'success' | 'danger' | 'warning' | 'primary' | string
}>()

const classes = computed(() => [
  'inline-flex items-center rounded-md px-2 py-0.5 text-xs font-medium',
  props.type === 'success' && 'bg-emerald-50 text-emerald-700 ring-1 ring-emerald-200',
  props.type === 'danger' && 'bg-red-50 text-red-700 ring-1 ring-red-200',
  props.type === 'warning' && 'bg-amber-50 text-amber-700 ring-1 ring-amber-200',
  props.type === 'primary' && 'bg-blue-50 text-blue-700 ring-1 ring-blue-200',
  (!props.type || props.type === 'info') && 'bg-stone-100 text-zinc-700 ring-1 ring-stone-200'
])
</script>
