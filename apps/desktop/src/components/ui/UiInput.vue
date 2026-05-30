<template>
  <textarea
    v-if="type === 'textarea'"
    v-bind="$attrs"
    :value="modelValue ?? ''"
    :rows="rows || 3"
    :placeholder="placeholder ? t(placeholder) : undefined"
    :disabled="disabled"
    class="min-h-20 w-full resize-y rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none transition placeholder:text-zinc-400 focus:border-zinc-900 focus:ring-4 focus:ring-zinc-900/10 disabled:cursor-not-allowed disabled:bg-stone-100 disabled:text-zinc-500"
    @input="emitValue"
  />
  <input
    v-else
    v-bind="$attrs"
    :type="type || 'text'"
    :value="modelValue ?? ''"
    :placeholder="placeholder ? t(placeholder) : undefined"
    :disabled="disabled"
    class="min-h-9 w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none transition placeholder:text-zinc-400 focus:border-zinc-900 focus:ring-4 focus:ring-zinc-900/10 disabled:cursor-not-allowed disabled:bg-stone-100 disabled:text-zinc-500"
    @input="emitValue"
  />
</template>

<script setup lang="ts">
import { t } from '@/i18n'

defineOptions({ inheritAttrs: false })

defineProps<{
  modelValue?: string | number
  type?: string
  rows?: number
  placeholder?: string
  disabled?: boolean
  clearable?: boolean
  showPassword?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: string]
}>()

function emitValue(event: Event) {
  emit('update:modelValue', (event.target as HTMLInputElement | HTMLTextAreaElement).value)
}
</script>
