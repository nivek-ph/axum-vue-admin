<script lang="ts">
import { defineComponent, inject, onBeforeUnmount, onMounted, reactive, watch, type PropType } from 'vue'

export type UiOptionValue = string | number | boolean | null

export interface UiSelectOption {
  id: symbol
  label: string
  value: UiOptionValue
}

export interface UiSelectContext {
  register(option: UiSelectOption): void
  unregister(id: symbol): void
}

export const uiSelectKey = Symbol('UiSelect')

export default defineComponent({
  name: 'UiOption',
  props: {
    label: String,
    value: {
      type: [String, Number, Boolean, null] as unknown as PropType<UiOptionValue>,
      default: null
    }
  },
  setup(props) {
    const context = inject<UiSelectContext | null>(uiSelectKey, null)
    const id = Symbol('UiOption')
    const option = reactive<UiSelectOption>({
      id,
      label: props.label ?? String(props.value ?? ''),
      value: props.value
    })

    watch(
      () => [props.label, props.value] as const,
      ([label, value]) => {
        option.label = label ?? String(value ?? '')
        option.value = value
      }
    )

    onMounted(() => context?.register(option))
    onBeforeUnmount(() => context?.unregister(id))

    return () => null
  }
})
</script>
