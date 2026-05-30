<template>
  <span class="inline-block w-full">
    <span class="hidden">
      <slot />
    </span>
    <select
      v-bind="$attrs"
      :multiple="multiple"
      :value="selectedValue"
      class="min-h-9 w-full rounded-lg border border-stone-300 bg-white px-3 py-2 text-sm text-zinc-900 outline-none transition focus:border-zinc-900 focus:ring-4 focus:ring-zinc-900/10"
      @change="updateValue"
    >
      <option v-if="clearable || placeholder" value="-1">{{ t(placeholder || 'Select') }}</option>
      <option v-for="(option, index) in options" :key="String(option.id)" :value="String(index)">
        {{ t(option.label) }}
      </option>
    </select>
  </span>
</template>

<script setup lang="ts">
import { computed, provide, ref } from 'vue'
import { t } from '@/i18n'
import { uiSelectKey, type UiOptionValue, type UiSelectContext, type UiSelectOption } from './UiOption.vue'

defineOptions({ inheritAttrs: false })

const props = defineProps<{
  modelValue?: UiOptionValue | UiOptionValue[]
  placeholder?: string
  multiple?: boolean
  clearable?: boolean
  filterable?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: UiOptionValue | UiOptionValue[] | undefined]
}>()

const options = ref<UiSelectOption[]>([])

provide<UiSelectContext>(uiSelectKey, {
  register(option) {
    options.value = [...options.value.filter((item) => item.id !== option.id), option]
  },
  unregister(id) {
    options.value = options.value.filter((item) => item.id !== id)
  }
})

const selectedValue = computed(() => {
  if (props.multiple) {
    return Array.isArray(props.modelValue)
      ? props.modelValue.map((value) => String(options.value.findIndex((option) => option.value === value)))
      : []
  }

  return String(options.value.findIndex((option) => option.value === props.modelValue))
})

function updateValue(event: Event) {
  const select = event.target as HTMLSelectElement
  if (props.multiple) {
    const selected = Array.from(select.selectedOptions)
      .map((option) => options.value[Number(option.value)]?.value)
      .filter((value) => value !== undefined)
    emit('update:modelValue', selected)
    return
  }

  const index = Number(select.value)
  emit('update:modelValue', index >= 0 ? options.value[index]?.value : undefined)
}
</script>
