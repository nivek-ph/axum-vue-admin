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
import { cva } from 'class-variance-authority'
import { t } from '@/i18n'
import { cn } from '@/lib/utils'

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

const buttonVariants = cva(
  'inline-flex items-center justify-center whitespace-nowrap text-sm font-semibold transition disabled:pointer-events-none disabled:opacity-60',
  {
    variants: {
      variant: {
        default: 'min-h-8 rounded-lg border border-stone-300 bg-white px-3 text-zinc-900 hover:border-stone-400 hover:bg-stone-50',
        primary: 'min-h-8 rounded-lg border border-zinc-900 bg-zinc-900 px-3 text-white hover:bg-zinc-800',
        danger: 'min-h-8 rounded-lg border border-red-500 bg-red-500 px-3 text-white hover:bg-red-600',
        warning: 'min-h-8 rounded-lg border border-amber-500 bg-amber-500 px-3 text-white hover:bg-amber-600',
        success: 'min-h-8 rounded-lg border border-emerald-500 bg-emerald-500 px-3 text-white hover:bg-emerald-600',
        link: 'rounded-md px-1'
      },
      tone: {
        default: '',
        primary: '',
        danger: ''
      }
    },
    compoundVariants: [
      { variant: 'link', tone: 'primary', class: 'text-blue-600 hover:text-blue-700 hover:underline' },
      { variant: 'link', tone: 'danger', class: 'text-red-600 hover:text-red-700 hover:underline' },
      { variant: 'link', tone: 'default', class: 'text-zinc-700 hover:text-zinc-950 hover:underline' }
    ],
    defaultVariants: {
      variant: 'default',
      tone: 'default'
    }
  }
)

const classes = computed(() => {
  const variant = props.link ? 'link' : props.type
  const tone = props.type === 'primary' || props.type === 'danger' ? props.type : 'default'
  return cn(buttonVariants({ variant: variant as any, tone }))
})
</script>
