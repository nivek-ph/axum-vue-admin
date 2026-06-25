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
  'inline-flex min-h-10 items-center justify-center whitespace-nowrap rounded-md px-4 py-2 text-sm font-medium transition focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-zinc-950/10 disabled:pointer-events-none disabled:opacity-50',
  {
    variants: {
      variant: {
        default: 'border border-stone-300 bg-white text-zinc-900 hover:bg-stone-50',
        primary: 'border border-zinc-900 bg-zinc-900 text-white hover:bg-zinc-800',
        danger: 'border border-red-600 bg-red-600 text-white hover:bg-red-700',
        warning: 'border border-amber-600 bg-amber-600 text-white hover:bg-amber-700',
        success: 'border border-emerald-600 bg-emerald-600 text-white hover:bg-emerald-700',
        link: 'min-h-0 border border-transparent bg-transparent px-1 py-0'
      },
      tone: {
        default: '',
        primary: '',
        danger: ''
      }
    },
    compoundVariants: [
      { variant: 'link', tone: 'primary', class: 'text-zinc-900 hover:text-zinc-950 hover:underline' },
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
