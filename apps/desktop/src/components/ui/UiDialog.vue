<template>
  <div
    v-if="modelValue"
    class="fixed inset-0 z-50 grid place-items-center bg-zinc-950/35 p-4"
    @click="$emit('update:modelValue', false)"
  >
    <section
      class="max-h-[90vh] w-full overflow-hidden rounded-2xl border border-stone-200 bg-white shadow-2xl"
      :style="{ maxWidth: width || '560px' }"
      @click.stop
    >
      <header class="flex items-center justify-between gap-4 border-b border-stone-200 px-5 py-4">
        <h2 class="text-lg font-bold text-zinc-900">{{ title ? t(title) : '' }}</h2>
        <button
          type="button"
          class="grid h-8 w-8 place-items-center rounded-lg text-xl text-zinc-500 transition hover:bg-stone-100 hover:text-zinc-900"
          @click="$emit('update:modelValue', false)"
        >
          ×
        </button>
      </header>
      <div class="max-h-[65vh] overflow-y-auto px-5 py-4">
        <slot />
      </div>
      <footer v-if="$slots.footer" class="flex justify-end gap-3 border-t border-stone-200 px-5 py-4">
        <slot name="footer" />
      </footer>
    </section>
  </div>
</template>

<script setup lang="ts">
import { t } from '@/i18n'

defineProps<{
  modelValue?: boolean
  title?: string
  width?: string
}>()

defineEmits<{
  'update:modelValue': [value: boolean]
}>()
</script>
