import type { App, InjectionKey, PropType, Slots } from 'vue'
import {
  computed,
  defineComponent,
  h,
  inject,
  nextTick,
  onBeforeUnmount,
  onMounted,
  provide,
  reactive,
  ref,
  watch
} from 'vue'

type OptionValue = string | number | boolean | null

interface SelectOption {
  id: symbol
  label: string
  value: OptionValue
}

interface SelectContext {
  register(option: SelectOption): void
  unregister(id: symbol): void
}

const selectKey: InjectionKey<SelectContext> = Symbol('CompatSelect')

function normalizeSize(value?: string | number) {
  if (value === undefined) return undefined
  return typeof value === 'number' ? `${value}px` : value
}

const ElButton = defineComponent({
  name: 'ElButton',
  props: {
    type: String,
    loading: Boolean,
    disabled: Boolean,
    link: Boolean
  },
  emits: ['click'],
  setup(props, { slots, emit, attrs }) {
    return () =>
      h(
        'button',
        {
          ...attrs,
          type: (attrs.type as string) || 'button',
          disabled: props.disabled || props.loading,
          class: [
            'compat-button',
            props.type ? `compat-button--${props.type}` : '',
            props.link ? 'compat-button--link' : '',
            attrs.class
          ],
          onClick: (event: MouseEvent) => emit('click', event)
        },
        props.loading ? ['处理中...'] : slots.default?.()
      )
  }
})

const ElCard = defineComponent({
  name: 'ElCard',
  setup(_, { slots, attrs }) {
    return () => h('section', { ...attrs, class: ['compat-card', attrs.class] }, slots.default?.())
  }
})

const ElForm = defineComponent({
  name: 'ElForm',
  setup(_, { slots, attrs }) {
    return () => h('form', attrs, slots.default?.())
  }
})

const ElFormItem = defineComponent({
  name: 'ElFormItem',
  props: {
    label: String
  },
  setup(props, { slots }) {
    return () =>
      h('label', { class: ['compat-form-item', props.label ? '' : 'compat-form-item--no-label'] }, [
        props.label ? h('span', { class: 'compat-form-label' }, props.label) : null,
        h('span', { class: 'compat-form-control' }, slots.default?.())
      ])
  }
})

const ElInput = defineComponent({
  name: 'ElInput',
  props: {
    modelValue: [String, Number] as PropType<string | number | undefined>,
    type: String,
    rows: Number,
    placeholder: String,
    disabled: Boolean
  },
  emits: ['update:modelValue'],
  setup(props, { emit, attrs }) {
    return () => {
      const common = {
        ...attrs,
        value: props.modelValue ?? '',
        placeholder: props.placeholder,
        disabled: props.disabled,
        class: ['compat-input', attrs.class],
        onInput: (event: Event) => {
          emit('update:modelValue', (event.target as HTMLInputElement | HTMLTextAreaElement).value)
        }
      }

      if (props.type === 'textarea') {
        return h('textarea', { ...common, rows: props.rows || 3 })
      }

      return h('input', { ...common, type: props.type || 'text' })
    }
  }
})

const ElInputNumber = defineComponent({
  name: 'ElInputNumber',
  props: {
    modelValue: Number,
    min: Number,
    precision: Number,
    disabled: Boolean
  },
  emits: ['update:modelValue'],
  setup(props, { emit, attrs }) {
    return () =>
      h('input', {
        ...attrs,
        type: 'number',
        value: props.modelValue ?? 0,
        min: props.min,
        disabled: props.disabled,
        class: ['compat-input', attrs.class],
        step: props.precision === 0 ? 1 : 'any',
        onInput: (event: Event) => {
          const value = Number((event.target as HTMLInputElement).value)
          emit('update:modelValue', Number.isFinite(value) ? value : 0)
        }
      })
  }
})

const ElSwitch = defineComponent({
  name: 'ElSwitch',
  props: {
    modelValue: Boolean,
    activeText: String,
    inactiveText: String
  },
  emits: ['update:modelValue'],
  setup(props, { emit }) {
    return () =>
      h('button', {
        type: 'button',
        class: ['compat-switch', props.modelValue ? 'compat-switch--on' : ''],
        onClick: () => emit('update:modelValue', !props.modelValue)
      }, [
        h('span', { class: 'compat-switch-thumb' }),
        h('span', { class: 'compat-switch-text' }, props.modelValue ? props.activeText : props.inactiveText)
      ])
  }
})

const ElOption = defineComponent({
  name: 'ElOption',
  props: {
    label: String,
    value: {
      type: [String, Number, Boolean, null] as unknown as PropType<OptionValue>,
      default: null
    }
  },
  setup(props) {
    const context = inject(selectKey, null)
    const id = Symbol('CompatOption')
    const option = reactive<SelectOption>({
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

const ElSelect = defineComponent({
  name: 'ElSelect',
  props: {
    modelValue: [String, Number, Boolean, Array, null] as unknown as PropType<OptionValue | OptionValue[]>,
    placeholder: String,
    multiple: Boolean,
    clearable: Boolean
  },
  emits: ['update:modelValue'],
  setup(props, { slots, emit, attrs }) {
    const options = ref<SelectOption[]>([])

    provide(selectKey, {
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

    function update(event: Event) {
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

    return () => {
      const optionNodes = [
        props.clearable || props.placeholder
          ? h('option', { value: '-1' }, props.placeholder || '请选择')
          : null,
        ...options.value.map((option, index) => h('option', { value: String(index) }, option.label))
      ]

      return h('span', { class: 'compat-select-shell' }, [
        slots.default?.(),
        h(
          'select',
          {
            ...attrs,
            multiple: props.multiple,
            value: selectedValue.value,
            class: ['compat-select', attrs.class],
            onChange: update
          },
          optionNodes
        )
      ])
    }
  }
})

interface TableColumn {
  id: symbol
  type?: string
  prop?: string
  label?: string
  width?: string | number
  minWidth?: string | number
  slots: Slots
}

interface TableContext {
  register(column: TableColumn): void
  unregister(id: symbol): void
}

const tableKey: InjectionKey<TableContext> = Symbol('CompatTable')

const ElTableColumn = defineComponent({
  name: 'ElTableColumn',
  props: {
    type: String,
    prop: String,
    label: String,
    width: [String, Number],
    minWidth: [String, Number]
  },
  setup(props, { slots }) {
    const context = inject(tableKey, null)
    const id = Symbol('CompatTableColumn')
    const column = reactive<TableColumn>({
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

function valueAt(row: Record<string, unknown>, prop?: string) {
  return prop ? row[prop] : ''
}

function flattenRows(rows: Record<string, unknown>[], level = 0): Array<{ row: Record<string, unknown>; level: number }> {
  return rows.flatMap((row) => {
    const children = Array.isArray(row.children) ? (row.children as Record<string, unknown>[]) : []
    return [{ row, level }, ...flattenRows(children, level + 1)]
  })
}

const ElTable = defineComponent({
  name: 'ElTable',
  props: {
    data: {
      type: Array as PropType<Record<string, unknown>[]>,
      default: () => []
    }
  },
  emits: ['current-change', 'selection-change'],
  setup(props, { slots, emit, attrs }) {
    const columns = ref<TableColumn[]>([])
    const selectedRows = ref<Record<string, unknown>[]>([])

    provide(tableKey, {
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

    return () => {
      const hiddenColumns = h('span', { style: 'display: none;' }, slots.default?.())
      const table = h('div', { class: 'compat-table-wrap' }, [
        h('table', { ...attrs, class: ['compat-table', attrs.class] }, [
          h('thead', [
            h('tr', columns.value.map((column) =>
              h('th', {
                style: {
                  width: normalizeSize(column.width),
                  minWidth: normalizeSize(column.minWidth)
                }
              }, column.type === 'selection'
                ? h('input', {
                    type: 'checkbox',
                    checked: rows.value.length > 0 && selectedRows.value.length === rows.value.length,
                    onChange: (event: Event) => setAllSelected((event.target as HTMLInputElement).checked)
                  })
                : column.label)
            ))
          ]),
          h('tbody', rows.value.length
            ? rows.value.map(({ row, level }) =>
                h('tr', { onClick: () => emit('current-change', row) }, columns.value.map((column, index) =>
                  h('td', {
                    style: {
                      width: normalizeSize(column.width),
                      minWidth: normalizeSize(column.minWidth),
                      paddingLeft: index === 0 && level ? `${level * 18 + 16}px` : undefined
                    }
                  }, column.type === 'selection'
                    ? h('input', {
                        type: 'checkbox',
                        checked: isSelected(row),
                        onClick: (event: MouseEvent) => event.stopPropagation(),
                        onChange: (event: Event) => setSelected(row, (event.target as HTMLInputElement).checked)
                      })
                    : column.slots.default
                      ? column.slots.default({ row })
                      : String(valueAt(row, column.prop) ?? ''))
                ))
              )
            : [h('tr', [h('td', { colspan: Math.max(columns.value.length, 1), class: 'compat-empty' }, '暂无数据')])])
        ])
      ])

      nextTick()
      return h('div', [hiddenColumns, table])
    }
  }
})

const ElTag = defineComponent({
  name: 'ElTag',
  props: {
    type: String
  },
  setup(props, { slots }) {
    return () => h('span', { class: ['compat-tag', props.type ? `compat-tag--${props.type}` : ''] }, slots.default?.())
  }
})

const ElDialog = defineComponent({
  name: 'ElDialog',
  props: {
    modelValue: Boolean,
    title: String,
    width: String
  },
  emits: ['update:modelValue'],
  setup(props, { slots, emit }) {
    return () => props.modelValue
      ? h('div', { class: 'compat-dialog-backdrop', onClick: () => emit('update:modelValue', false) }, [
          h('section', {
            class: 'compat-dialog',
            style: { width: props.width || '560px' },
            onClick: (event: MouseEvent) => event.stopPropagation()
          }, [
            h('header', { class: 'compat-dialog-header' }, [
              h('h2', props.title),
              h('button', { type: 'button', class: 'compat-dialog-close', onClick: () => emit('update:modelValue', false) }, '×')
            ]),
            h('div', { class: 'compat-dialog-body' }, slots.default?.()),
            slots.footer ? h('footer', { class: 'compat-dialog-footer' }, slots.footer()) : null
          ])
        ])
      : null
  }
})

const ElPagination = defineComponent({
  name: 'ElPagination',
  props: {
    total: Number,
    currentPage: {
      type: Number,
      default: 1
    },
    pageSize: {
      type: Number,
      default: 10
    }
  },
  emits: ['update:current-page', 'current-change'],
  setup(props, { emit }) {
    const totalPages = computed(() => Math.max(1, Math.ceil((props.total || 0) / props.pageSize)))

    function setPage(page: number) {
      const next = Math.min(Math.max(page, 1), totalPages.value)
      emit('update:current-page', next)
      emit('current-change', next)
    }

    return () => h('nav', { class: 'compat-pagination' }, [
      h('span', `共 ${props.total || 0} 条`),
      h('button', { type: 'button', disabled: props.currentPage <= 1, onClick: () => setPage(props.currentPage - 1) }, '上一页'),
      h('span', `${props.currentPage} / ${totalPages.value}`),
      h('button', { type: 'button', disabled: props.currentPage >= totalPages.value, onClick: () => setPage(props.currentPage + 1) }, '下一页')
    ])
  }
})

const ElAlert = defineComponent({
  name: 'ElAlert',
  props: {
    title: String,
    type: String,
    showIcon: Boolean,
    closable: {
      type: Boolean,
      default: true
    }
  },
  setup(props, { slots }) {
    return () => h('div', { class: ['compat-alert', props.type ? `compat-alert--${props.type}` : ''] }, [
      props.showIcon ? h('span', { class: 'compat-alert-icon' }, '!') : null,
      h('span', slots.default?.() ?? props.title)
    ])
  }
})

export const ElementCompat = {
  install(app: App) {
    app.component('ElAlert', ElAlert)
    app.component('ElButton', ElButton)
    app.component('ElCard', ElCard)
    app.component('ElDialog', ElDialog)
    app.component('ElForm', ElForm)
    app.component('ElFormItem', ElFormItem)
    app.component('ElInput', ElInput)
    app.component('ElInputNumber', ElInputNumber)
    app.component('ElOption', ElOption)
    app.component('ElPagination', ElPagination)
    app.component('ElSelect', ElSelect)
    app.component('ElSwitch', ElSwitch)
    app.component('ElTable', ElTable)
    app.component('ElTableColumn', ElTableColumn)
    app.component('ElTag', ElTag)
    app.directive('loading', {
      mounted(el, binding) {
        el.classList.toggle('compat-loading', Boolean(binding.value))
      },
      updated(el, binding) {
        el.classList.toggle('compat-loading', Boolean(binding.value))
      }
    })
  }
}
