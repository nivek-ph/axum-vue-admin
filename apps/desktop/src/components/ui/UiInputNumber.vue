<template>
  <input
    v-bind="$attrs"
    type="number"
    :value="modelValue ?? 0"
    :min="min"
    :step="precision === 0 ? 1 : 'any'"
    :disabled="disabled"
    class="min-h-9 w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none transition focus:border-zinc-900 focus:ring-4 focus:ring-zinc-900/10 disabled:cursor-not-allowed disabled:bg-stone-100 disabled:text-zinc-500"
    @input="emitValue"
  />
</template>

<script setup lang="ts">
defineOptions({ inheritAttrs: false })

defineProps<{
  modelValue?: number
  min?: number
  precision?: number
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: number]
}>()

function emitValue(event: Event) {
  const value = Number((event.target as HTMLInputElement).value)
  emit('update:modelValue', Number.isFinite(value) ? value : 0)
}
</script>
