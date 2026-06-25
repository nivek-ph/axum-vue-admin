<template>
  <button
    type="button"
    :class="[
      'inline-flex min-h-10 items-center gap-2 rounded-md border px-3 py-2 text-sm font-medium transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-950/10',
      modelValue ? 'border-zinc-900 bg-zinc-900 text-white' : 'border-stone-300 bg-white text-zinc-700 hover:bg-stone-50'
    ]"
    @click="$emit('update:modelValue', !modelValue)"
  >
    <span :class="['h-4 w-4 rounded-full shadow-sm ring-1 ring-stone-300 transition', modelValue ? 'bg-white' : 'bg-stone-200']" />
    <span>{{ t(modelValue ? activeText || '' : inactiveText || '') }}</span>
  </button>
</template>

<script setup lang="ts">
import { t } from '@/i18n'

defineProps<{
  modelValue?: boolean
  activeText?: string
  inactiveText?: string
}>()

defineEmits<{
  'update:modelValue': [value: boolean]
}>()
</script>
