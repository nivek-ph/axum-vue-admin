<template>
  <button
    v-bind="$attrs"
    :type="nativeType"
    :disabled="disabled || loading"
    :class="classes"
    @click="$emit('click', $event)"
  >
    <span v-if="loading">{{ t('Processing...') }}</span>
    <slot v-else />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { t } from '@/i18n'

defineOptions({ inheritAttrs: false })

const props = withDefaults(
  defineProps<{
    type?: 'default' | 'primary' | 'danger' | 'warning' | 'success' | string
    nativeType?: 'button' | 'submit' | 'reset'
    loading?: boolean
    disabled?: boolean
    link?: boolean
  }>(),
  {
    type: 'default',
    nativeType: 'button'
  }
)

defineEmits<{
  click: [event: MouseEvent]
}>()

const classes = computed(() => {
  if (props.link) {
    return [
      'inline-flex items-center justify-center whitespace-nowrap rounded-md px-1 text-sm font-semibold transition',
      'disabled:pointer-events-none disabled:opacity-60',
      props.type === 'primary' && 'text-blue-600 hover:text-blue-700 hover:underline',
      props.type === 'danger' && 'text-red-600 hover:text-red-700 hover:underline',
      props.type !== 'primary' && props.type !== 'danger' && 'text-zinc-700 hover:text-zinc-950 hover:underline'
    ]
  }

  return [
    'inline-flex min-h-8 items-center justify-center whitespace-nowrap rounded-lg border px-3 text-sm font-semibold transition',
    'disabled:pointer-events-none disabled:opacity-60',
    props.type === 'primary' && 'border-zinc-900 bg-zinc-900 text-white hover:bg-zinc-800',
    props.type === 'danger' && 'border-red-500 bg-red-500 text-white hover:bg-red-600',
    props.type === 'warning' && 'border-amber-500 bg-amber-500 text-white hover:bg-amber-600',
    props.type === 'success' && 'border-emerald-500 bg-emerald-500 text-white hover:bg-emerald-600',
    (!props.type || props.type === 'default') && 'border-stone-300 bg-white text-zinc-900 hover:border-stone-400 hover:bg-stone-50'
  ]
})
</script>
