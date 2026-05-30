<template>
  <nav class="flex flex-wrap items-center justify-end gap-3 text-sm text-zinc-600">
    <span>{{ t('Total') }} {{ total || 0 }} {{ t('items') }}</span>
    <UiButton :disabled="currentPage <= 1" @click="setPage(currentPage - 1)">{{ t('Previous') }}</UiButton>
    <span>{{ currentPage }} / {{ totalPages }}</span>
    <UiButton :disabled="currentPage >= totalPages" @click="setPage(currentPage + 1)">{{ t('Next') }}</UiButton>
  </nav>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { t } from '@/i18n'
import UiButton from './UiButton.vue'

const props = withDefaults(
  defineProps<{
    total?: number
    currentPage?: number
    pageSize?: number
    background?: boolean
    layout?: string
  }>(),
  {
    currentPage: 1,
    pageSize: 10
  }
)

const emit = defineEmits<{
  'update:current-page': [value: number]
  'current-change': [value: number]
}>()

const totalPages = computed(() => Math.max(1, Math.ceil((props.total || 0) / props.pageSize)))

function setPage(page: number) {
  const next = Math.min(Math.max(page, 1), totalPages.value)
  emit('update:current-page', next)
  emit('current-change', next)
}
</script>
