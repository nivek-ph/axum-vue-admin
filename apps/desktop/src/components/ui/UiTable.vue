<template>
  <div>
    <span class="hidden">
      <slot />
    </span>
    <div class="relative overflow-x-auto rounded-md border border-stone-200 bg-white">
      <div
        v-if="loading && rows.length > 0"
        class="absolute inset-x-0 top-0 z-10 h-0.5 overflow-hidden bg-stone-100"
      >
        <div class="h-full w-1/3 animate-pulse rounded-full bg-zinc-300" />
      </div>
      <table v-bind="$attrs" class="w-full border-collapse text-left text-sm">
        <thead class="bg-stone-50 text-xs font-medium text-zinc-600">
          <tr>
            <th
              v-for="column in columns"
              :key="String(column.id)"
              class="border-b border-stone-200 px-4 py-3"
              :style="{ width: normalizeSize(column.width), minWidth: normalizeSize(column.minWidth) }"
            >
              <input
                v-if="column.type === 'selection'"
                type="checkbox"
                :checked="rows.length > 0 && selectedRows.length === rows.length"
                @change="setAllSelected(($event.target as HTMLInputElement).checked)"
              />
              <span v-else>{{ column.label ? t(column.label) : '' }}</span>
            </th>
          </tr>
        </thead>
        <tbody :class="loading && rows.length > 0 ? 'pointer-events-none opacity-60' : ''">
          <tr v-if="loading && rows.length === 0">
            <td :colspan="Math.max(columns.length, 1)" class="px-4 py-8 text-center text-zinc-500">
              {{ t('Loading') }}...
            </td>
          </tr>
          <tr v-else-if="!rows.length">
            <td :colspan="Math.max(columns.length, 1)" class="px-4 py-8 text-center text-zinc-500">
              {{ t('No data') }}
            </td>
          </tr>
          <tr
            v-for="{ row, level } in rows"
            v-else
            :key="rowKey(row, level)"
            class="border-b border-stone-100 last:border-b-0 hover:bg-stone-50/70"
            @click="$emit('current-change', row)"
          >
            <td
              v-for="(column, index) in columns"
              :key="String(column.id)"
              class="px-4 py-3 text-zinc-800"
              :style="{
                width: normalizeSize(column.width),
                minWidth: normalizeSize(column.minWidth),
                paddingLeft: index === 0 && level ? `${level * 18 + 16}px` : undefined
              }"
            >
              <input
                v-if="column.type === 'selection'"
                type="checkbox"
                :checked="isSelected(row)"
                @click.stop
                @change="setSelected(row, ($event.target as HTMLInputElement).checked)"
              />
              <component v-else-if="column.slots.default" :is="column.slots.default" :row="row" />
              <span v-else>{{ valueAt(row, column.prop) }}</span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, provide, ref, watch } from 'vue'
import { t } from '@/i18n'
import { uiTableKey, type UiTableColumnDef, type UiTableContext } from './UiTableColumn.vue'

defineOptions({ inheritAttrs: false })

const props = withDefaults(
  defineProps<{
    data?: Record<string, unknown>[]
    loading?: boolean
  }>(),
  {
    data: () => []
  }
)

const emit = defineEmits<{
  'current-change': [row: Record<string, unknown>]
  'selection-change': [rows: Record<string, unknown>[]]
}>()

const columns = ref<UiTableColumnDef[]>([])
const selectedRows = ref<Record<string, unknown>[]>([])

provide<UiTableContext>(uiTableKey, {
  register(column) {
    columns.value = [...columns.value.filter((item) => item.id !== column.id), column]
  },
  unregister(id) {
    columns.value = columns.value.filter((item) => item.id !== id)
  }
})

const rows = computed(() => flattenRows(props.data))

watch(
  () => props.data,
  () => {
    selectedRows.value = []
    emit('selection-change', [])
  }
)

function flattenRows(input: Record<string, unknown>[], level = 0): Array<{ row: Record<string, unknown>; level: number }> {
  return input.flatMap((row) => {
    const children = Array.isArray(row.children) ? (row.children as Record<string, unknown>[]) : []
    return [{ row, level }, ...flattenRows(children, level + 1)]
  })
}

function normalizeSize(value?: string | number) {
  if (value === undefined) return undefined
  return typeof value === 'number' ? `${value}px` : value
}

function valueAt(row: Record<string, unknown>, prop?: string) {
  return prop ? String(row[prop] ?? '') : ''
}

function rowKey(row: Record<string, unknown>, level: number) {
  return String(row.id ?? row.id ?? row.authorityId ?? row.path ?? row.name ?? `${level}-${props.data.indexOf(row)}`)
}

function isSelected(row: Record<string, unknown>) {
  return selectedRows.value.includes(row)
}

function setSelected(row: Record<string, unknown>, checked: boolean) {
  selectedRows.value = checked
    ? [...selectedRows.value, row]
    : selectedRows.value.filter((item) => item !== row)
  emit('selection-change', selectedRows.value)
}

function setAllSelected(checked: boolean) {
  selectedRows.value = checked ? rows.value.map((item) => item.row) : []
  emit('selection-change', selectedRows.value)
}
</script>
