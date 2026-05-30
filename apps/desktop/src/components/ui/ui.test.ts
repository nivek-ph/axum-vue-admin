import { mount } from '@vue/test-utils'
import { nextTick } from 'vue'
import { describe, expect, it } from 'vitest'

import UiButton from './UiButton.vue'
import UiDialog from './UiDialog.vue'
import UiInput from './UiInput.vue'
import UiSelect from './UiSelect.vue'
import UiTable from './UiTable.vue'
import UiTableColumn from './UiTableColumn.vue'
import UiTag from './UiTag.vue'

describe('ui primitives', () => {
  it('emits clicks from UiButton', async () => {
    const wrapper = mount(UiButton, {
      slots: {
        default: 'Save'
      }
    })

    await wrapper.get('button').trigger('click')

    expect(wrapper.emitted('click')).toHaveLength(1)
  })

  it('updates UiInput model value', async () => {
    const wrapper = mount(UiInput, {
      props: {
        modelValue: ''
      }
    })

    await wrapper.get('input').setValue('admin')

    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual(['admin'])
  })

  it('updates UiSelect model value', async () => {
    const wrapper = mount(UiSelect, {
      props: {
        modelValue: ''
      },
      slots: {
        default: '<UiOption label="Enabled" value="enabled" />'
      },
      global: {
        components: {
          UiOption: (await import('./UiOption.vue')).default
        }
      }
    })

    await wrapper.get('select').setValue('0')

    expect(wrapper.emitted('update:modelValue')?.[0]).toEqual(['enabled'])
  })

  it('renders UiDialog only when open', () => {
    const wrapper = mount(UiDialog, {
      props: {
        modelValue: true,
        title: 'EditUser'
      },
      slots: {
        default: 'Form content'
      }
    })

    expect(wrapper.text()).toContain('EditUser')
    expect(wrapper.text()).toContain('Form content')
  })

  it('renders UiTable columns and scoped cells', async () => {
    const wrapper = mount(UiTable, {
      props: {
        data: [{ name: 'admin', enable: 1 }]
      },
      slots: {
        default: `
          <UiTableColumn prop="name" label="Username" />
          <UiTableColumn label="Status">
            <template #default="{ row }">
              <UiTag :type="row.enable === 1 ? 'success' : 'danger'">Enabled</UiTag>
            </template>
          </UiTableColumn>
        `
      },
      global: {
        components: {
          UiTableColumn,
          UiTag
        }
      }
    })

    await nextTick()

    expect(wrapper.text()).toContain('Username')
    expect(wrapper.text()).toContain('admin')
    expect(wrapper.text()).toContain('Enabled')
  })
})
