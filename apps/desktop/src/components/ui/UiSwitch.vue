<template>
  <button
    type="button"
    :class="[
      'inline-flex min-h-8 items-center gap-2 rounded-full border px-2 py-1 text-sm font-semibold transition',
      modelValue ? 'border-zinc-900 bg-zinc-900 text-white' : 'border-stone-300 bg-stone-100 text-zinc-700'
    ]"
    @click="$emit('update:modelValue', !modelValue)"
  >
    <span :class="['h-5 w-5 rounded-full bg-white shadow transition', modelValue ? 'translate-x-1' : '']" />
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
