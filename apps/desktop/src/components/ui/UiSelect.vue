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
      <span class="ui-select-chevron">⌄</span>
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
          <span>{{ t(placeholder || 'Select') }}</span>
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
          <span v-if="isSelected(option.value)" class="ui-select-check">✓</span>
        </button>
        <div v-if="options.length === 0" class="ui-select-empty">{{ t('No options') }}</div>
      </div>
    </Teleport>
  </span>
</template>

<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, provide, ref } from 'vue'
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
  const preferredHeight = Math.min(240, Math.max(48, itemCount * 44 + 12))
  const gap = 6
  const sidePadding = 8
  const availableAbove = Math.max(0, rect.top - sidePadding)
  const availableBelow = Math.max(0, viewportHeight - rect.bottom - sidePadding)
  const shouldOpenUp = availableBelow < preferredHeight + gap && availableAbove > availableBelow
  const availableSpace = shouldOpenUp ? availableAbove : availableBelow
  const maxHeight = Math.max(48, Math.min(240, availableSpace - gap))
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
  min-height: 42px;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  border: 1px solid rgba(60, 60, 67, 0.22);
  border-radius: 14px;
  background: linear-gradient(180deg, #ffffff 0%, #fbfbfd 100%);
  padding: 8px 12px;
  color: #1d1d1f;
  box-shadow: inset 0 1px 0 rgba(255, 255, 255, 0.85);
  text-align: left;
  cursor: pointer;
  transition: border-color 0.16s ease, box-shadow 0.16s ease, background 0.16s ease;
}

.ui-select-trigger:hover,
.ui-select-trigger:focus-visible {
  border-color: rgba(0, 122, 255, 0.5);
  background: #ffffff;
  box-shadow: 0 0 0 4px rgba(0, 122, 255, 0.11), inset 0 1px 0 rgba(255, 255, 255, 0.9);
  outline: none;
}

.ui-select-trigger:disabled {
  cursor: not-allowed;
  background: #f5f5f7;
  color: rgba(60, 60, 67, 0.45);
}

.ui-select-value {
  min-width: 0;
  overflow: hidden;
  color: #1d1d1f;
  font-size: 14px;
  font-weight: 650;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.ui-select-value.is-placeholder,
.ui-select-chevron {
  color: rgba(60, 60, 67, 0.62);
}

.ui-select-chevron {
  flex: 0 0 auto;
  font-size: 18px;
}

.ui-select-menu {
  z-index: 80;
  display: grid;
  gap: 6px;
  max-height: 240px;
  overflow-y: auto;
  border: 1px solid rgba(60, 60, 67, 0.18);
  border-radius: 16px;
  background: rgba(255, 255, 255, 0.96);
  padding: 6px;
  box-shadow: 0 18px 42px rgba(0, 0, 0, 0.12), 0 2px 8px rgba(0, 0, 0, 0.06);
  backdrop-filter: saturate(180%) blur(18px);
}

.ui-select-option {
  display: flex;
  width: 100%;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  border: 0;
  border-radius: 10px;
  background: transparent;
  padding: 10px;
  color: #1d1d1f;
  font-size: 14px;
  font-weight: 600;
  text-align: left;
  cursor: pointer;
  transition: background 0.14s ease, color 0.14s ease;
}

.ui-select-option:hover {
  background: rgba(0, 0, 0, 0.04);
}

.ui-select-option:focus-visible {
  background: rgba(0, 122, 255, 0.08);
  outline: none;
}

.ui-select-option.is-selected {
  background: rgba(0, 122, 255, 0.1);
  color: #005ecb;
}

.ui-select-check {
  color: #007aff;
  font-weight: 900;
}

.ui-select-empty {
  padding: 14px;
  color: rgba(60, 60, 67, 0.62);
  font-size: 14px;
  text-align: center;
}
</style>
