<template>
  <span ref="rootRef" v-bind="$attrs" class="ui-select">
    <span class="ui-select-options">
      <slot />
    </span>
    <button
      ref="triggerRef"
      data-test="ui-select-trigger"
      class="ui-select-trigger"
      type="button"
      :disabled="disabled"
      :aria-expanded="open"
      @click="toggleMenu"
      @keydown.escape.prevent="closeMenu"
    >
      <span :class="['ui-select-value', !hasValue && 'is-placeholder']">{{ displayLabel }}</span>
      <ChevronDown class="ui-select-chevron" aria-hidden="true" />
    </button>
    <Teleport to="body">
      <div
        v-if="open"
        data-test="ui-select-menu"
        class="ui-select-menu"
        role="listbox"
        :aria-multiselectable="multiple || undefined"
        :style="menuStyle"
      >
        <button
          v-if="showClearOption"
          data-test="ui-select-clear"
          class="ui-select-option"
          type="button"
          role="option"
          :aria-selected="!hasValue"
          @click="clearValue"
        >
          <span>{{ clearOptionLabel }}</span>
        </button>
        <button
          v-for="(option, index) in options"
          :key="String(option.id)"
          :data-test="`ui-select-option-${index}`"
          :class="['ui-select-option', isSelected(option.value) && 'is-selected']"
          type="button"
          role="option"
          :aria-selected="isSelected(option.value)"
          @click="selectOption(option.value)"
        >
          <span>{{ t(option.label) }}</span>
          <Check v-if="isSelected(option.value)" class="ui-select-check" aria-hidden="true" />
        </button>
        <div v-if="options.length === 0" class="ui-select-empty">{{ t('No options') }}</div>
      </div>
    </Teleport>
  </span>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, provide, ref } from 'vue'
import { Check, ChevronDown } from '@lucide/vue'
import { t } from '@/i18n'
import { uiSelectKey, type UiOptionValue, type UiSelectContext, type UiSelectOption } from './UiOption.vue'

defineOptions({ inheritAttrs: false })

const props = defineProps<{
  modelValue?: UiOptionValue | UiOptionValue[]
  placeholder?: string
  multiple?: boolean
  clearable?: boolean
  filterable?: boolean
  disabled?: boolean
}>()

const emit = defineEmits<{
  'update:modelValue': [value: UiOptionValue | UiOptionValue[] | undefined]
}>()

const options = ref<UiSelectOption[]>([])
const open = ref(false)
const rootRef = ref<HTMLElement | null>(null)
const triggerRef = ref<HTMLElement | null>(null)
const menuStyle = ref<Record<string, string>>({})

provide<UiSelectContext>(uiSelectKey, {
  register(option) {
    options.value = [...options.value.filter((item) => item.id !== option.id), option]
  },
  unregister(id) {
    options.value = options.value.filter((item) => item.id !== id)
  }
})

const selectedOptions = computed(() => {
  if (props.multiple) {
    const values = Array.isArray(props.modelValue) ? props.modelValue : []
    return options.value.filter((option) => values.includes(option.value))
  }

  return options.value.filter((option) => option.value === props.modelValue)
})

const hasValue = computed(() => selectedOptions.value.length > 0)
const showClearOption = computed(() => !props.multiple && (props.clearable || props.placeholder))
const clearOptionLabel = computed(() => props.placeholder ? t('All {label}', { label: t(props.placeholder) }) : t('Select'))
const displayLabel = computed(() => {
  if (selectedOptions.value.length > 0) {
    return selectedOptions.value.map((option) => t(option.label)).join(', ')
  }

  return t(props.placeholder || 'Select')
})

function toggleMenu() {
  if (props.disabled) return
  open.value = !open.value
  if (open.value) {
    nextTick(updateMenuPosition)
  }
}

function closeMenu() {
  open.value = false
}

function isSelected(value: UiOptionValue) {
  if (props.multiple) {
    return Array.isArray(props.modelValue) && props.modelValue.includes(value)
  }

  return props.modelValue === value
}

function selectOption(value: UiOptionValue) {
  if (props.multiple) {
    const current = Array.isArray(props.modelValue) ? [...props.modelValue] : []
    const next = current.includes(value) ? current.filter((item) => item !== value) : [...current, value]
    emit('update:modelValue', next)
    return
  }

  emit('update:modelValue', value)
  closeMenu()
}

function clearValue() {
  emit('update:modelValue', undefined)
  closeMenu()
}

function handleDocumentClick(event: MouseEvent) {
  const target = event.target as Node
  if (!rootRef.value?.contains(target) && !(target instanceof HTMLElement && target.closest('[data-test="ui-select-menu"]'))) {
    closeMenu()
  }
}

function updateMenuPosition() {
  const trigger = triggerRef.value
  if (!trigger) return

  const rect = trigger.getBoundingClientRect()
  const viewportHeight = window.innerHeight || document.documentElement.clientHeight
  const viewportWidth = window.innerWidth || document.documentElement.clientWidth
  const itemCount = options.value.length + (showClearOption.value ? 1 : 0)
  const preferredHeight = Math.min(224, Math.max(40, itemCount * 36 + 8))
  const gap = 4
  const sidePadding = 8
  const availableAbove = Math.max(0, rect.top - sidePadding)
  const availableBelow = Math.max(0, viewportHeight - rect.bottom - sidePadding)
  const shouldOpenUp = availableBelow < preferredHeight + gap && availableAbove > availableBelow
  const availableSpace = shouldOpenUp ? availableAbove : availableBelow
  const maxHeight = Math.max(40, Math.min(224, availableSpace - gap))
  const menuHeight = Math.min(preferredHeight, maxHeight)
  const top = shouldOpenUp
    ? Math.max(sidePadding, rect.top - menuHeight - gap)
    : Math.min(rect.bottom + gap, viewportHeight - sidePadding - menuHeight)
  const left = Math.min(Math.max(rect.left, sidePadding), Math.max(sidePadding, viewportWidth - rect.width - sidePadding))

  menuStyle.value = {
    position: 'fixed',
    top: `${top}px`,
    left: `${left}px`,
    width: `${rect.width}px`,
    maxHeight: `${maxHeight}px`
  }
}

onMounted(() => {
  document.addEventListener('click', handleDocumentClick)
  window.addEventListener('resize', updateMenuPosition)
  window.addEventListener('scroll', updateMenuPosition, true)
})

onBeforeUnmount(() => {
  document.removeEventListener('click', handleDocumentClick)
  window.removeEventListener('resize', updateMenuPosition)
  window.removeEventListener('scroll', updateMenuPosition, true)
})
</script>

<style scoped>
.ui-select {
  position: relative;
  display: inline-block;
  width: 100%;
}

.ui-select-options {
  display: none;
}

.ui-select-trigger {
  display: flex;
  width: 100%;
  min-height: 40px;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  border: 1px solid #d6d3d1;
  border-radius: 6px;
  background: #ffffff;
  padding: 8px 12px;
  color: #18181b;
  text-align: left;
  cursor: pointer;
  transition: border-color 0.16s ease, box-shadow 0.16s ease, color 0.16s ease;
}

.ui-select-trigger:hover,
.ui-select-trigger:focus-visible {
  border-color: #a8a29e;
  box-shadow: 0 0 0 2px rgba(24, 24, 27, 0.1);
  outline: none;
}

.ui-select-trigger:disabled {
  cursor: not-allowed;
  background: #f5f5f4;
  color: #71717a;
}

.ui-select-value {
  min-width: 0;
  overflow: hidden;
  color: #18181b;
  font-size: 14px;
  font-weight: 500;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ui-select-value.is-placeholder,
.ui-select-chevron {
  color: #71717a;
}

.ui-select-chevron {
  flex: 0 0 auto;
  width: 16px;
  height: 16px;
  opacity: 0.72;
}

.ui-select-menu {
  z-index: 80;
  display: grid;
  gap: 2px;
  max-height: 224px;
  overflow-y: auto;
  border: 1px solid #e7e5e4;
  border-radius: 6px;
  background: #ffffff;
  padding: 4px;
  box-shadow: 0 10px 24px rgba(24, 24, 27, 0.12);
}

.ui-select-option {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  border: 0;
  border-radius: 4px;
  background: transparent;
  padding: 8px;
  color: #18181b;
  font-size: 14px;
  font-weight: 500;
  text-align: left;
  cursor: pointer;
  transition: background 0.14s ease, color 0.14s ease;
}

.ui-select-option:hover {
  background: #f5f5f4;
}

.ui-select-option:focus-visible {
  background: #f5f5f4;
  outline: none;
}

.ui-select-option.is-selected {
  background: #f5f5f4;
  color: #18181b;
}

.ui-select-check {
  width: 16px;
  height: 16px;
  color: #18181b;
}

.ui-select-empty {
  padding: 14px;
  color: rgba(60, 60, 67, 0.62);
  font-size: 14px;
  text-align: center;
}
</style>
