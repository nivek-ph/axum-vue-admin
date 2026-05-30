<script lang="ts">
import { defineComponent, inject, onBeforeUnmount, onMounted, reactive, watch, type PropType, type Slots } from 'vue'

export interface UiTableColumnDef {
  id: symbol
  type?: string
  prop?: string
  label?: string
  width?: string | number
  minWidth?: string | number
  slots: Slots
}

export interface UiTableContext {
  register(column: UiTableColumnDef): void
  unregister(id: symbol): void
}

export const uiTableKey = Symbol('UiTable')

export default defineComponent({
  name: 'UiTableColumn',
  props: {
    type: String,
    prop: String,
    label: String,
    width: [String, Number] as PropType<string | number>,
    minWidth: [String, Number] as PropType<string | number>
  },
  setup(props, { slots }) {
    const context = inject<UiTableContext | null>(uiTableKey, null)
    const id = Symbol('UiTableColumn')
    const column = reactive<UiTableColumnDef>({
      id,
      type: props.type,
      prop: props.prop,
      label: props.label,
      width: props.width,
      minWidth: props.minWidth,
      slots
    })

    watch(
      () => [props.type, props.prop, props.label, props.width, props.minWidth] as const,
      ([type, prop, label, width, minWidth]) => {
        column.type = type
        column.prop = prop
        column.label = label
        column.width = width
        column.minWidth = minWidth
      }
    )

    onMounted(() => context?.register(column))
    onBeforeUnmount(() => context?.unregister(id))

    return () => null
  }
})
</script>
